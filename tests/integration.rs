use {
    base64::prelude::*,
    fireblocks_signer_transport::*,
    solana_client::rpc_client::RpcClient,
    solana_client::rpc_config::CommitmentConfig,
    solana_keypair::Keypair,
    solana_sdk::{
        instruction::Instruction, message::Message, pubkey::Pubkey, signature::Signature,
        signer::Signer, transaction::Transaction,
    },
    spl_memo_interface::{instruction::build_memo, v3::ID as MEMO_PROGRAM_ID},
    std::{
        env,
        str::FromStr,
        sync::{Arc, Once},
        time::Duration,
    },
    tracing::field::Empty,
    tracing_subscriber::{EnvFilter, fmt::format::FmtSpan},
};

static MEMO_MESSAGE: &str = "signed by https://github.com/carteraMesh/fireblocks-signer-transport";
pub static INIT: Once = Once::new();
pub fn memo(message: &str, signers: &[&Pubkey]) -> Instruction {
    build_memo(&MEMO_PROGRAM_ID, message.as_bytes(), signers)
}

#[allow(clippy::unwrap_used, clippy::missing_panics_doc)]
pub fn setup() {
    INIT.call_once(|| {
        tracing_subscriber::fmt()
            .with_target(true)
            .with_level(true)
            .with_span_events(FmtSpan::CLOSE)
            .with_env_filter(EnvFilter::from_default_env())
            .init();

        if env::var("CI").is_err() {
            // only load .env if not in CI
            let env = dotenvy::dotenv_override();
            if env.is_err() {
                tracing::debug!("no .env file");
            }
        }
    });
}

fn client() -> anyhow::Result<(Client, Arc<RpcClient>)> {
    let api_key: String =
        std::env::var("FIREBLOCKS_API_KEY").expect("FIREBLOCKS_API_KEY is not set");
    let key: String = std::env::var("FIREBLOCKS_SECRET").expect("FIREBLOCKS_SECRET is not set");
    let rsa_pem = key.as_bytes().to_vec();
    let url = std::env::var("RPC_URL")?;
    tracing::info!("RPC_URL: {url}");
    let rpc = Arc::new(RpcClient::new(url));
    let url: String =
        std::env::var("FIREBLOCKS_ENDPOINT").unwrap_or(FIREBLOCKS_SANDBOX_API.to_string());
    Ok((
        ClientBuilder::new(&api_key, &rsa_pem)
            .with_url(url)
            .with_user_agent("fireblocks-solana-signer-test")
            .with_timeout(Duration::from_secs(15))
            .build()?,
        rpc,
    ))
}

#[test]
fn test_program_call() -> anyhow::Result<()> {
    setup();
    let (client, rpc) = client()?;
    let span = tracing::info_span!("test_program_call", pk = Empty);
    let _g = span.enter();
    let pk = Pubkey::from_str(&client.address("0", "SOL_TEST")?)?;
    span.record("pk", pk.to_string());
    tracing::info!("testing normal program_call");
    let hash = rpc.get_latest_blockhash()?;
    let ins = vec![memo(MEMO_MESSAGE, &[&pk])];
    let tx = Transaction::new_unsigned(Message::new_with_blockhash(&ins, Some(&pk), &hash));
    let base64_tx = BASE64_STANDARD.encode(bincode::serialize(&tx)?);
    let resp = client.program_call("SOL_TEST", "0", base64_tx)?;
    tracing::info!("txid {resp}");
    let (resp, sig) = client.poll(
        &resp.id,
        std::time::Duration::from_secs(90),
        Duration::from_secs(7),
        |t| tracing::info!("transaction status {t}"),
    )?;
    assert!(sig.is_some());
    let sig = Signature::from_str(&sig.unwrap_or_default())?;
    tracing::info!("sig {sig} txid {}", resp.id);
    let confirmed = rpc.confirm_transaction_with_commitment(&sig, CommitmentConfig::confirmed())?;
    assert!(confirmed.value);
    Ok(())
}

#[test]
fn test_sign_only() -> anyhow::Result<()> {
    setup();
    let span = tracing::info_span!("test_sign_only", pk = Empty);
    let _g = span.enter();
    let stake_signer = Keypair::new();
    let stake_account = stake_signer.pubkey();
    let (client, rpc) = client()?;
    let pk = Pubkey::from_str(&client.address("0", "SOL_TEST")?)?;
    span.record("pk", pk.to_string());
    tracing::info!("using additional signer {stake_account}");
    let hash = rpc.get_latest_blockhash()?;
    let ins = vec![memo(MEMO_MESSAGE, &[&pk, &stake_account])];
    let message = Message::new_with_blockhash(&ins, Some(&pk), &hash);
    let mut tx = Transaction::new_unsigned(message);
    tx.partial_sign(&[stake_signer], hash);
    let base64_tx = BASE64_STANDARD.encode(bincode::serialize(&tx)?);
    let resp = client.sign_only("SOL_TEST", "0", base64_tx)?;
    tracing::info!("txid {resp}");
    let (resp, sig) = client.poll(
        &resp.id,
        std::time::Duration::from_secs(90),
        Duration::from_secs(7),
        |t| tracing::info!("transaction status {t}"),
    )?;
    assert!(sig.is_some());
    let sig = sig.unwrap_or_default();
    tracing::info!("signOnly: sig {sig} txid {}", resp.id);
    let decoded: Vec<u8> = bs58::decode(&sig).into_vec()?;
    let array: [u8; 64] = decoded
        .try_into()
        .map_err(|_| anyhow::format_err!("Invalid signature"))?;

    tx.signatures[0] = Signature::from(array);
    assert!(tx.is_signed());
    rpc.send_and_confirm_transaction(&tx)?;
    Ok(())
}

#[test]
fn test_invalid_api_key() {
    let result = ClientBuilder::new("not-a-uuid", b"fake-secret").build();
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("API key must be a valid UUID"));
}

#[test]
fn test_invalid_secret_key() {
    let result = ClientBuilder::new(
        "550e8400-e29b-41d4-a716-446655440000",
        b"not-a-valid-pem-key",
    )
    .build();
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Secret key format is invalid"));
    assert!(err.to_string().contains("BEGIN RSA PRIVATE KEY"));
}

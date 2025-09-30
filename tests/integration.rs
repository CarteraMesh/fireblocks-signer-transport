use {
    base64::prelude::*,
    fireblocks_signer_transport::*,
    solana_native_token::sol_str_to_lamports,
    solana_pubkey::Pubkey,
    solana_rpc_client::rpc_client::RpcClient,
    solana_sdk::{
        instruction::Instruction,
        message::Message,
        signature::{Keypair, Signature},
        signer::Signer,
        transaction::Transaction,
    },
    solana_stake_interface::{
        self,
        instruction::{self as stake_instruction},
        state::{Authorized, Lockup},
    },
    std::{
        env,
        str::FromStr,
        sync::{Arc, Once},
        time::Duration,
    },
    tracing_subscriber::{EnvFilter, fmt::format::FmtSpan},
};

pub static INIT: Once = Once::new();
pub fn memo(message: &str) -> Instruction {
    Instruction {
        program_id: spl_memo::id(),
        accounts: vec![],
        data: message.as_bytes().to_vec(),
    }
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
            let env = dotenvy::dotenv();
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
    let rpc = Arc::new(RpcClient::new(
        std::env::var("RPC_URL")
            .ok()
            .unwrap_or("https://rpc.ankr.com/solana_devnet".to_string()),
    ));

    Ok((
        ClientBuilder::new(&api_key, &rsa_pem)
            .with_sandbox()
            .with_user_agent("fireblocks-solana-signer-test")
            .with_timeout(Duration::from_secs(15))
            .build()?,
        rpc,
    ))
}

#[test]
fn test_client() -> anyhow::Result<()> {
    setup();
    let (client, rpc) = client()?;
    let pk = Pubkey::from_str(&client.address("0", "SOL_TEST")?)?;
    tracing::info!("using pubkey {}", pk);
    let hash = rpc.get_latest_blockhash()?;
    let message = Message::new_with_blockhash(&[memo("fireblocks signer")], Some(&pk), &hash);
    let tx = Transaction::new_unsigned(message);
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
    let sig = sig.unwrap_or_default();
    tracing::info!("sig {sig} txid {}", resp.id);
    Ok(())
}

#[test]
fn test_sign_only() -> anyhow::Result<()> {
    setup();
    let stake_signer = Keypair::new();
    let stake_account = stake_signer.pubkey();
    let (client, rpc) = client()?;
    let pk = Pubkey::from_str(&client.address("0", "SOL_TEST")?)?;
    tracing::info!("using pubkey {pk} {stake_account}");
    let hash = rpc.get_latest_blockhash()?;
    let authorized = Authorized::auto(&pk);
    let mut inxs = stake_instruction::create_account(
        &pk,
        &stake_account,
        &authorized,
        &Lockup::default(),
        sol_str_to_lamports("1.4").ok_or(anyhow::format_err!("oh no"))?,
    );
    inxs.push(memo("only sign"));
    let message = Message::new_with_blockhash(&inxs, Some(&pk), &hash);
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
    let decoded: Vec<u8> = solana_sdk::bs58::decode(&sig).into_vec()?;
    let array: [u8; 64] = decoded
        .try_into()
        .map_err(|_| anyhow::format_err!("Invalid signature"))?;

    tx.signatures[0] = Signature::from(array);
    assert!(tx.is_signed());
    let result = rpc.simulate_transaction(&tx)?;
    assert!(result.value.err.is_none());
    Ok(())
}

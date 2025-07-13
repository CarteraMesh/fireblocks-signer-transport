use serde::{Deserialize, Serialize};
mod create_transaction_response;
mod extra_parameters;
mod source_transfer_peer_path;
mod system_message_info;
mod transaction_operation;
mod transaction_request;
mod transaction_response;
mod transaction_status;
mod transaction_sub_status;
mod transfer_peer_path_type;
pub use {
    create_transaction_response::*, extra_parameters::ExtraParameters,
    source_transfer_peer_path::*, system_message_info::*,
    transaction_operation::TransactionOperation, transaction_request::*, transaction_response::*,
    transaction_status::*, transaction_sub_status::*,
};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct VaultWalletAddress {
    #[serde(rename = "assetId")]
    pub asset_id: String,
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct VaultAddressesResponse {
    pub addresses: Vec<VaultWalletAddress>,
}

#[cfg(test)]
mod test {
    use {super::*, serde_json::json};
    #[test]
    fn vault_addresses() -> anyhow::Result<()> {
        let raw = json!({
            "addresses": [
                {
                    "assetId": "SOL",
                    "address": "FdtiepBtP98oU2uPNgAzUoGwggUDdRXwJH2KJo3oUaix"
                }
            ]
        });
        let expected = "FdtiepBtP98oU2uPNgAzUoGwggUDdRXwJH2KJo3oUaix";
        let parsed: VaultAddressesResponse = serde_json::from_value(raw)?;
        assert_eq!(parsed.addresses.len(), 1);
        assert_eq!(parsed.addresses[0].address, expected);
        Ok(())
    }
}

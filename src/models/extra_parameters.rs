use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExtraParameters {
    #[serde(rename = "programCallData")]
    pub program_call_data: String,
    #[serde(rename = "signOnly", skip_serializing_if = "Option::is_none")]
    pub sign_only: Option<bool>,
    #[serde(rename = "useDurableNonce", skip_serializing_if = "Option::is_none")]
    pub use_durable_nonce: Option<bool>,
}

impl ExtraParameters {
    pub fn new(program_call_data: String) -> Self {
        Self {
            program_call_data,
            use_durable_nonce: None,
            sign_only: None,
        }
    }
}

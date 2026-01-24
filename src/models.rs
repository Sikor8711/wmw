use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomerData {
    #[serde(rename = "firstname")]
    pub first_name: String,
    pub email: String,
}

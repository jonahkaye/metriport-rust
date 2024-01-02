use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Address {
    #[serde(rename = "addressLine1")]
    pub address_line_1: String,
    #[serde(rename = "addressLine2")]
    pub address_line_2: Option<String>,
    pub city: String,
    pub state: String,
    pub zip: String,
    pub country: String,
}
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Address {
    pub address_line_1: String,
    pub city: String,
    pub state: String,
    pub zip: String,
}
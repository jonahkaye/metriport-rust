use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Address {
    street: String,
    city: String,
    state: String,
    zip: String,
    // Include other fields as needed
}
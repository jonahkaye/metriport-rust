use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct BaseUpdate {
    id: String,
    e_tag: Option<String>,
}

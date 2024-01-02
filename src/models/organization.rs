use serde_derive::{Serialize, Deserialize};
use crate::models::commons::address::Address;

#[derive(Serialize, Deserialize, Debug)]
pub enum OrgType {
    #[serde(rename = "acuteCare")]
    AcuteCare,
    #[serde(rename = "ambulatory")]
    Ambulatory,
    #[serde(rename = "hospital")]
    Hospital,
    #[serde(rename = "labSystems")]
    LabSystems,
    #[serde(rename = "pharmacy")]
    Pharmacy,
    #[serde(rename = "postAcuteCare")]
    PostAcuteCare,
    // Include other types as needed
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrganizationCreate(pub String, pub OrgType, pub Address);

#[derive(Serialize, Deserialize, Debug)]
pub struct Organization {
    id: String,
    #[serde(rename = "eTag")]
    e_tag: String,
    oid: String,
    name: String,
    #[serde(rename = "type")]
    org_type: OrgType,
    location: Address,
}
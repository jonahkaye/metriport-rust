use serde_derive::{Serialize, Deserialize};
use crate::models::commons::address::Address;
use crate::models::commons::base_update::BaseUpdate;

#[derive(Serialize, Deserialize)]
pub enum OrgType {
    AcuteCare,
    Ambulatory,
    Hospital,
    LabSystems,
    Pharmacy,
    PostAcuteCare,
    // Include other types as needed
}

#[derive(Serialize, Deserialize)]
pub struct OrganizationCreate(String, OrgType, Address);


#[derive(Serialize, Deserialize)]
pub struct Organization {
    #[serde(flatten)]
    base: BaseUpdate,
    oid: String,
    #[serde(flatten)]
    create_data: OrganizationCreate,
}
// tests/integration_test.rs
use metriport_rust::{MetriportSDK, Options, models::organization::OrganizationCreate, models::commons::address::Address, models::organization::OrgType};
use std::collections::HashMap;

#[tokio::test]
async fn test_create_organization() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = "you_api_key".to_string();

    let options = Options {
        timeout: Some(30),
        additional_headers: Some(HashMap::new()),
        sandbox: Some(true),
        base_address: None,
    };

    let sdk = MetriportSDK::new(api_key, options);

    let address = Address {
        address_line_1: "1211 Edris Dr".to_string(),
        city: "LA".to_string(),
        state: "CA".to_string(),
        zip: "90035".to_string(),
    };

    let org_data = OrganizationCreate("Bayle Inc".to_string(), OrgType::AcuteCare, address);

    let _organization = sdk.create_organization(org_data).await?;


    Ok(())
}
// tests/integration_test.rs
use metriport_rust::{MetriportSDK, Options};
use std::collections::HashMap;
use dotenv::dotenv;
use std::env;


#[tokio::test]
async fn test_get_organization() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY must be set");

    let options = Options {
        timeout: Some(30),
        additional_headers: Some(HashMap::new()),
        sandbox: Some(true),
        base_address: None,
    };

    let sdk = MetriportSDK::new(api_key, options);

    let _organization = sdk.get_organization().await?;
    println!("organization {:?}", _organization);
    
    Ok(())
}
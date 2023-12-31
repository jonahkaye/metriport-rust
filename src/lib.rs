pub mod models;

use reqwest::Client;
use serde_derive::{Deserialize, Serialize};
use std::error::Error;
use std::collections::HashMap;


use crate::models::organization::{Organization, OrganizationCreate};

// Define your types
#[derive(Serialize, Deserialize)]
pub struct Resource {
    // Fields of the resource
}

//const BASE_PATH: &str = "/medical/v1";
const ORGANIZATION_URL: &str = "/organization";
// const FACILITY_URL: &str = "/facility";
// const PATIENT_URL: &str = "/patient";
// const DOCUMENT_URL: &str = "/document";
const BASE_ADDRESS: &str = "http://localhost:8080";
const BASE_ADDRESS_SANDBOX: &str = "https://sandbox.metriport.com";

pub struct MetriportSDK {
    client: Client,
    base_url: String,
    api_key: String,
}

#[allow(dead_code)]
pub struct Options {
    pub timeout: Option<u64>,
    pub additional_headers: Option<HashMap<String, String>>,
    pub sandbox: Option<bool>,
    pub base_address: Option<String>,
}

impl MetriportSDK {
    pub fn new(api_key: String, options: Options) -> Self {
        // Set base_url based on options.sandbox and options.base_address
        let base_url = match (options.sandbox, options.base_address) {
            (Some(true), _) => String::from(BASE_ADDRESS_SANDBOX),
            (_, Some(address)) => address,
            _ => String::from(BASE_ADDRESS),
        };

        Self {
            client: Client::new(),
            base_url,
            api_key,
        }
    }

    pub async fn create_organization(&self, data: OrganizationCreate) -> Result<Organization, Box<dyn Error>> {
        let url = format!("{}{}", self.base_url, ORGANIZATION_URL);
        let response: reqwest::Response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&data)
            .send()
            .await?;
    
        if response.status().is_success() {
            let organization: Organization = response.json().await?;
            Ok(organization)
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to create organization",
            )))
        }
    }
}
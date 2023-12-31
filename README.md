```
let options = Options {
    timeout: None,
    additional_headers: None,
    sandbox: None,
    base_address: Some(String::from("localhost:3000")),
};

let api_key = String::from("your-api-key");

let sdk = SDK::new(api_key, options);

let organization = sdk.create_organization(data).await?;

```
cargo build

cargo login your-token
cargo publish --dry-run
cargo publish
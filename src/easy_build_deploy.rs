use std::fs;
use reqwest::header::{Authorization, Bearer};
use serde_json::{json, Value};

fn deploy_file(deployment_server_api_url: &str, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Read file into base64 UTF-8 string
    let filename = filepath;
    let contents = fs::read(filename)?;
    let base64_contents = base64::encode(&contents);
    let body = json!({
        "file": base64_contents
    });

    // Create client and authorization header
    let client = reqwest::Client::new();
    let access_token = "YOUR_ACCESS_TOKEN";
    let auth_header = Authorization(Bearer { token: access_token.to_owned() });

    // Send POST request with body and authorization header
    let response = client.post("https://example.com/api")
        .header(auth_header)
        .json(&body)
        .send()?;
    
    // Process response
    let response_body: Value = response.json()?;
    println!("{:#?}", response_body);

    Ok(())
}
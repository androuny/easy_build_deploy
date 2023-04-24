use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::{BufReader, Read};
use reqwest::blocking::{Client};
use reqwest::header::{HeaderName};
use serde_json::{json};
use std::path::Path;
use std::io::Write;

#[derive(Serialize, Deserialize)]
struct Config {
    api_url: String,
    access_token: String,
    file_path: String,
    filename: String,
}

fn init_config() -> Config{
    let path = "easy_config.json";
    if !Path::new(&path).exists() {
        // If it doesn't exist, create it and write some initial JSON to it
        let mut file = File::create(&path).expect("Failed to create a config file");
        // Write default config to the file
        let default_config = r#"{
            "api_endpoint": "api url to the server",
            "access_token": "access token for upload",
            "file_path": "path to your file that shall be uploaded to server api",
            "filename" : "name for the file to be uploaded",
        }"#;
        file.write_all(default_config.as_bytes()).expect("Failed to write default config to file");
    }

    let file = File::open(path).expect("Failed to open config file");
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let config: Config = serde_json::from_reader(reader).expect("Failed to read config file.");
    return config;

    // Return the `User`.
    
}

fn format_bytes(bytes: u64) -> String {
    if bytes < 1024 {
        return format!("{} B", bytes);
    }

    let units = ["KB", "MB", "GB", "TB"];
    let mut size = bytes as f64 / 1024.0;
    let mut unit = 0;

    while size > 1024.0 {
        size /= 1024.0;
        unit += 1;
    }

    format!("{:.2} {}", size, units[unit])
}

fn deploy_file(filepath: &str, access_token: &str, api_url: &str, filename: &str) {
    // Read file into base64 UTF-8 string
    let mut contents = File::open(filepath).expect("Failed to read deployable binary.");
    let mut buffer = Vec::new();
    contents.read_to_end(&mut buffer).unwrap();
    let base64_contents = base64::encode(buffer);
    println!("base64 string len: {} bytes - {}", base64_contents.len(), format_bytes(base64_contents.len() as u64));
    let body = json!({
        "file": base64_contents,
        "filename" : filename
    });

    // Create client and authorization header
    let client = Client::new();
    //let auth_header = Authorization(Bearer { token: access_token.to_owned() });
    let mut headers = reqwest::header::HeaderMap::new();
    let name: HeaderName = "Authorization".parse().unwrap();
    let token_string = format!("Bearer {}", access_token);
    headers.insert(name, token_string.parse().unwrap());


    // Send POST request with body and authorization header
    let response =  match client.post(api_url)
        .headers(headers)
        .json(&body)
        .send() 
        {
            Ok(respone) => respone.text().unwrap(),
            Err(err) => panic!("Error: {}", err)
        };

    // Process response
    println!("Response: {:?}", response);
    //let response_body: Value = response.json();
    //println!("{:#?}", response_body);
}


fn main() {
    let config = init_config();
    println!("local config loaded successfully");
    deploy_file(&config.file_path, &config.access_token, &config.api_url, &config.filename);
}

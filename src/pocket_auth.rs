use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;

pub fn write_access_file(token: &str, path: &str) {
    let mut file = File::create(path).unwrap();
    writeln!(&mut file, "{}", token).unwrap();
}

pub fn read_access_file(path: &str) -> Option<String> {
    let result = File::open(path);
    match result {
        Ok(_) => {
            let mut output = String::new();
            result.unwrap().read_to_string(&mut output).expect("Could not read access_token file");
            Some(output.trim().to_string())
        },
        Err(_) => None
    }
}

fn obtain_pocket_token(key: &str) -> String {
    let mut map = HashMap::new();
    map.insert("consumer_key", key);
    map.insert("redirect_uri", "pocketapp1234:authorizationFinished");

    let mut headers = HeaderMap::new();
    headers.insert("X-Accept", HeaderValue::from_static("application/json"));

    let client = reqwest::blocking::Client::new();
    let token_response : String = client.post("https://getpocket.com/v3/oauth/request")
        .json(&map)
        .headers(headers)
        .send()
        .expect("error requesting token from pocket")
        .text()
        .expect("error parsing response from pocket");

    println!("{:?}", token_response);

    let response : RequestResponse =
        serde_json::from_str(token_response.as_str())
        .expect("Could not parse the pocket API json");

    response.code
}

fn authenticate_pocket(token: &str, port: u16) {
    let callback : String = format!("127.0.0.1:{}", port);
    let callback_http : String = format!("http://{}", callback);

    let url : String = format!("https://getpocket.com/auth/authorize?request_token={}&redirect_uri={}", token, callback_http);
    webbrowser::open(url.as_str()).expect("Could not open up browser");

    println!("Waiting for response at {}", callback);

    let listener = TcpListener::bind(callback).unwrap();
    for _ in listener.incoming() {
        println!("Got incoming!");
        break;
    }
}

fn get_access_token(key: &str, code: &str) -> String {
    println!("Obtaining access token");

    let mut map = HashMap::new();
    map.insert("consumer_key", key);
    map.insert("code", code);

    let mut headers = HeaderMap::new();
    headers.insert("X-Accept", HeaderValue::from_static("application/json"));

    let client = reqwest::blocking::Client::new();
    let token_response : String = client.post("https://getpocket.com/v3/oauth/authorize")
        .json(&map)
        .headers(headers)
        .send()
        .expect("error requesting token from pocket")
        .text()
        .expect("error parsing response from pocket");

    println!("{:?}", token_response);

    let response : AccessResponse =
        serde_json::from_str(token_response.as_str())
        .expect("Could not parse the pocket API json");

    response.access_token
}

pub fn read_access_token(path: &str, consumer_key: &str, port: u16) -> String {
    match read_access_file(path) {
        Some(token) => {
            token.to_string()
        },
        None => {
            let code = obtain_pocket_token(consumer_key);

            println!("Consumer key: {}", consumer_key);
            println!("Code: {}", code.as_str());
            authenticate_pocket(code.as_str(), port);

            let token : String = get_access_token(consumer_key, code.as_str());
            write_access_file(token.as_str(), path);

            token.to_string()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct AccessResponse {
    access_token: String,
    username: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct RequestResponse {
    code: String,
    state: Option<String>
}

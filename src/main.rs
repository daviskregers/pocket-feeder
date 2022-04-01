use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use serde_json;
use serde_xml_rs::from_str;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::fs;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use webbrowser;

mod item;
mod source;
mod config;
use crate::item::Item;
use crate::source::Source;
use crate::config::Config;

extern crate serde_yaml;

mod pocket_api;

const HTTP_PORT : u16 = 13372;
const ACCESS_TOKEN_FILE : &str = ".access_token";

#[derive(Serialize, Deserialize, Debug)]
struct Feed {
    channel: Channel,
}

impl Feed {
    fn filtered_items(self: &Self, source: &Source) -> Vec<Item> {
        self.channel
            .item
            .clone()
            .into_iter()
            .filter(|item| match source.item_included(item) {
                Some(true) => true,
                Some(false) => false,
                _ => true,
            })
            .collect()
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Channel {
    item: Vec<Item>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
struct RequestResponse {
    code: String,
    state: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct AccessResponse {
    access_token: String,
    username: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PocketItemResponse {
    status: u8,
    complete: u8,
    list: HashMap<String, PocketItem>,
}

impl PocketItemResponse {
    fn has_link(self: Self, link: String) -> bool {
        for (_k, v) in self.list {
            if v.given_url == link {
                return true;
            }
        }
        return false;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PocketItem {
    given_url: String,
    given_title: String,
}

fn get_config() -> Config {
    let file: String = env::current_dir().unwrap().display().to_string() + "/sources.yml";
    println!("Reading file: {}", file);

    let sources_text: String =
        fs::read_to_string(file).expect("Something went wrong when reading the file: {}");

    serde_yaml::from_str(&sources_text).expect("Something went wrong when parsing sources.yaml")
}

fn get_feed(source: &Source) -> Feed {
    let feed: String = reqwest::blocking::get(source.src.as_str())
        .expect(format!("Error getting feed from {}", source.src).as_str())
        .text()
        .expect(format!("Error parsing response from {}", source.src).as_str());

    from_str(feed.as_str())
        .expect(format!("Could not parse the RSS response for {}", source.src).as_str())
}

fn write_output(items: &Channel) {
    let serialized_items: String = serde_yaml::to_string(&items).expect("Error serializing items");

    println!("Writing all the items into output.yml file");

    let mut file = fs::File::create("output.yml")
        .expect("Error while creating or emptying the output.yml file");

    file.write_all(serialized_items.as_bytes())
        .expect("Error while writing to output.yml");
}

fn obtain_pocket_token(key: String) -> String {
    let mut map = HashMap::new();
    map.insert("consumer_key", key);
    map.insert("redirect_uri", "pocketapp1234:authorizationFinished".to_string());

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

fn authenticate_pocket(token: &str) {
    let callback : String = format!("127.0.0.1:{}", HTTP_PORT);
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


fn read_access_token(consumer_key: String) -> String {
    match read_access_file() {
        Some(token) => {
            token
        },
        None => {
            let code: String = obtain_pocket_token(consumer_key.clone());

            println!("Consumer key: {}", consumer_key);
            println!("Code: {}", code);
            authenticate_pocket(code.as_str());

            let token : String = get_access_token(consumer_key.as_str(), code.as_str());
            write_access_file(token.clone());

            token
        }
    }
}

fn read_access_file() -> Option<String> {
    let result = File::open(ACCESS_TOKEN_FILE);
    match result {
        Ok(_) => {
            let mut output = String::new();
            result.unwrap().read_to_string(&mut output).expect("Could not read access_token file");
            Some(output.trim().to_string())
        },
        Err(_) => None
    }
}

fn write_access_file(token: String) {
    let mut file = File::create(ACCESS_TOKEN_FILE).unwrap();
    writeln!(&mut file, "{}", token).unwrap();
}

fn read_pocket_items(key: String, access_token: String) -> PocketItemResponse {
    let mut map = HashMap::new();
    map.insert("consumer_key", key);
    map.insert("access_token", access_token);
    map.insert("state", "all".to_string());

    let mut headers = HeaderMap::new();
    headers.insert("X-Accept", HeaderValue::from_static("application/json"));

    let client = reqwest::blocking::Client::new();
    let token_response : String = client.post("https://getpocket.com/v3/get")
        .json(&map)
        .headers(headers)
        .send()
        .expect("error requesting item from pocket")
        .text()
        .expect("error parsing response from pocket");

    let response : PocketItemResponse =
        serde_json::from_str(token_response.as_str())
        .expect("Could not parse the pocket API json");

    response
}


fn main() {
    let sources: Config = get_config();
    let _guard;

    match sources.sentry {
        Some(s) => {
            match s.dsn {
                Some(dsn) => {
                    if ! dsn.is_empty() {
                        println!("Started up sentry!, {}", dsn);
                        _guard = sentry::init((dsn, sentry::ClientOptions {
                            release: sentry::release_name!(),
                            ..Default::default()
                        }));
                    } else {
                        println!("Sentry DSN empty, ignoring...");
                    }
                },
                _ => {
                    println!("Sentry DSN not set, ignoring...");
                },
            }
        },
        _ => {
            println!("Sentry configuration not set, ignoring ...");
        }
    }

    let consumer_key : String = sources.pocket.consumer;
    let access_token = read_access_token(consumer_key.clone());
    println!("Consumer token: {}", consumer_key);
    println!("Access token: {}", access_token);
    let pocket_items = read_pocket_items(consumer_key.clone(), access_token.clone());
    if pocket_items.complete != 1 {
        panic!("TODO: implement complete pocket item list");
    }

    let mut items: Channel = Channel { item: vec![] };

    for source in &sources.sources {
        let content: Feed = get_feed(source);
        for item in &mut content.filtered_items(source) {
            item.add_timestamps();
            if pocket_items.clone().has_link(item.link.clone()) {
                println!("Item {} is in pocket, skipping...", item.link);
            } else {
                println!("Item {} isnt in pocket, adding!", item.link);
                pocket_api::publish_pocket_item(consumer_key.clone(), access_token.clone(), item.clone(), source.name.clone());
            }
            items.item.push(item.clone())
        }
    }
    items.item.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
    write_output(&items);
}

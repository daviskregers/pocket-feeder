use std::env;
use std::fs;
use std::io::Write;

mod config;
mod feed;
mod item;
mod pocket_auth;
mod source;
mod pocket_api;
mod pocket_item_response;
mod pocket_item;

use crate::config::Config;
use crate::feed::{Feed, Channel};
use crate::pocket_item::PocketItem;

extern crate serde_yaml;

const HTTP_PORT : u16 = 13372;
const ACCESS_TOKEN_FILE : &str = ".access_token";
const CONFIG_FILE : &str = "/config.yml";

fn write_output(items: &Channel) {
    let serialized_items: String = serde_yaml::to_string(&items).expect("Error serializing items");

    println!("Writing all the items into output.yml file");

    let mut file = fs::File::create("output.yml")
        .expect("Error while creating or emptying the output.yml file");

    file.write_all(serialized_items.as_bytes())
        .expect("Error while writing to output.yml");
}

fn main() {
    let path: String = env::current_dir().unwrap().display().to_string() + CONFIG_FILE;
    let config: Config = config::get_config(&path);
    let _guard;

    match config.sentry {
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

    let consumer_key : &str = config.pocket.consumer.as_str();
    let access_token = pocket_auth::read_access_token(ACCESS_TOKEN_FILE, consumer_key, HTTP_PORT);
    println!("Consumer token: {}", consumer_key);
    println!("Access token: {}", access_token);
    let pocket_items = pocket_api::read_pocket_items(consumer_key, access_token.as_str());
    if pocket_items.complete != 1 {
        panic!("TODO: implement complete pocket item list");
    }

    let mut items: Channel = Channel { item: vec![] };

    for source in &config.sources {
        let content: Feed = source::get_feed(source);
        for item in &mut content.filtered_items(source) {
            item.add_timestamps();
            if pocket_items.clone().has_link(&item.link) {
                println!("Item {} is in pocket, skipping...", item.link);
            } else {
                println!("Item {} isnt in pocket, adding!", item.link);
                pocket_api::publish_pocket_item(consumer_key, access_token.as_str(), item.clone(), source.name.as_str());
            }
            items.item.push(item.clone())
        }
    }
    items.item.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
    write_output(&items);
}

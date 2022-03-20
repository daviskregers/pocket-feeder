use serde::{Serialize, Deserialize};
use serde_xml_rs::from_str;
use std::env;
use std::fs;
use std::io::Write;
use chrono::{DateTime, FixedOffset};

extern crate serde_yaml;

#[derive(Serialize, Deserialize, Debug)]
struct Source {
    src: String
}

#[derive(Serialize, Deserialize, Debug)]
struct SourceList {
    sources: Vec<Source>
}

#[derive(Serialize, Deserialize, Debug)]
struct Feed {
    channel: ItemList
}

#[derive(Serialize, Deserialize, Debug)]
struct ItemList {
    item: Vec<Item>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Item {
    author: Vec<String>,
    category: Option<Vec<String>>,
    description: String,
    link: String,
    pubDate: String,
    timestamp: Option<i64>,
    utc: Option<String>,
    title: String,
}

fn main() {
    let file : String = env::current_dir().unwrap().display().to_string() + "/sources.yml";
    println!("Reading file: {}", file);

    let sources_text : String = fs::read_to_string(file)
            .expect("Something went wrong when reading the file: {}");

    let sources : SourceList = serde_yaml::from_str(&sources_text)
                .expect("Something went wrong when parsing sources.yaml");

    let mut items : ItemList = ItemList { item : vec![] };

    for source in &sources.sources {
        let feed : String = reqwest::blocking::get(source.src.as_str())
            .expect(format!("Error getting feed from {}", source.src).as_str())
            .text()
            .expect(format!("Error parsing response from {}", source.src).as_str());

        let content : Feed = from_str(feed.as_str())
            .expect(format!("Could not parse the RSS response for {}", source.src).as_str());

        for item in &content.channel.item {
            items.item.push(item.clone());
        }
    }

    for item in &mut items.item {
        let datetime = DateTime::parse_from_rfc2822(item.pubDate.as_str())
            .expect("Error while parsing pubDate")
            .naive_utc();

        item.timestamp = Some(datetime.timestamp());
        item.utc = Some(datetime.to_string());
    }

    items.item.sort_by(|a, b| {
        a.timestamp.cmp(&b.timestamp)
    });

    let serialized_items : String = serde_yaml::to_string(&items)
            .expect("Error serializing items");

    println!("Got aggregated items: {:?}", serialized_items);

    let mut file = fs::File::create("output.yml")
        .expect("Error while creating or emptying the output.yml file");
    file.write_all(serialized_items.as_bytes())
        .expect("Error while writing to output.yml");
}

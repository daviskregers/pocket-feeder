use chrono::DateTime;
use serde::{Deserialize, Serialize};
use serde_xml_rs::from_str;
use std::env;
use std::fs;
use std::io::Write;

extern crate serde_yaml;

#[derive(Serialize, Deserialize, Debug)]
struct Source {
    src: String,
    exclude: Option<ExcludeRules>,
}

impl Source {
    fn item_included(self: &Self, item: &Item) -> Option<bool> {
        let mut excluded = false;

        for item_author in &item.author {
            if self
                .exclude
                .as_ref()?
                .author
                .as_ref()?
                .contains(&item_author)
            {
                excluded = true;
                break;
            }
        }

        for item_category in &item.category.clone()? {
            if self
                .exclude
                .as_ref()?
                .category
                .as_ref()?
                .contains(&item_category)
            {
                excluded = true;
                break;
            }
        }

        Some(!excluded)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct ExcludeRules {
    category: Option<Vec<String>>,
    author: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct SourceList {
    sources: Vec<Source>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Feed {
    channel: ItemList,
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
struct ItemList {
    item: Vec<Item>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
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

impl Item {
    fn add_timestamps(self: &mut Self) {
        let datetime = DateTime::parse_from_rfc2822(self.pubDate.as_str())
            .expect("Error while parsing pubDate")
            .naive_utc();

        self.timestamp = Some(datetime.timestamp());
        self.utc = Some(datetime.to_string());
    }
}

fn get_sources() -> SourceList {
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

fn write_output(items: &ItemList) {
    let serialized_items: String = serde_yaml::to_string(&items).expect("Error serializing items");

    println!("Writing all the items into output.yml file");

    let mut file = fs::File::create("output.yml")
        .expect("Error while creating or emptying the output.yml file");

    file.write_all(serialized_items.as_bytes())
        .expect("Error while writing to output.yml");
}

fn main() {
    let sources: SourceList = get_sources();
    let mut items: ItemList = ItemList { item: vec![] };

    for source in &sources.sources {
        let content: Feed = get_feed(source);
        for item in &mut content.filtered_items(source) {
            item.add_timestamps();
            items.item.push(item.clone())
        }
    }
    items.item.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
    write_output(&items);
}

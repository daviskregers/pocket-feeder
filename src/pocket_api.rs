use crate::item::Item;
use crate::pocket_item_response::PocketItemResponse;
use reqwest::header::{HeaderMap, HeaderValue};
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    #[test]
    fn publish_item_adds_rss_feeder_tag() {}
    fn publish_item_adds_source_name_tag() {}
    fn publish_item_adds_category_tags() {}
    fn publish_item_posts_to_pocket_api() {}
    fn writes_access_key() {}
    fn reads_access_key() {}
    fn reads_access_token() {}
}

pub fn publish_pocket_item(key: &str, access_token: &str, item: Item, source_name: &str) {
    let mut map = HashMap::new();
    map.insert("consumer_key", key);
    map.insert("access_token", access_token);
    map.insert("url", &item.link);
    map.insert("title", &item.title);

    let mut categories : Vec<String> = vec!();
    match item.category {
        Some(cats) => {
            for cat in cats {
                categories.push(cat);
            }
        },
        None => {}
    }
    categories.push("RSS feeder".to_string());
    categories.push(source_name.to_string());

    let cats = categories.join(",");

    map.insert("tags", cats.as_str());

    let mut headers = HeaderMap::new();
    headers.insert("X-Accept", HeaderValue::from_static("application/json"));

    let client = reqwest::blocking::Client::new();
    client.post("https://getpocket.com/v3/add")
        .json(&map)
        .headers(headers)
        .send()
        .expect("error requesting item from pocket")
        .text()
        .expect("error parsing response from pocket");
}

pub fn read_pocket_items(key: &str, access_token: &str) -> PocketItemResponse {
    let mut map = HashMap::new();
    map.insert("consumer_key", key.to_string());
    map.insert("access_token", access_token.to_string());
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

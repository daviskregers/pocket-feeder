use std::collections::HashMap;
use reqwest::header::{HeaderMap, HeaderValue};

use crate::item::Item;

#[cfg(test)]
mod tests {
    #[test]
    fn publish_item_adds_rss_feeder_tag() {}
    fn publish_item_adds_source_name_tag() {}
    fn publish_item_adds_category_tags() {}
    fn publish_item_posts_to_pocket_api() {}
}

pub fn publish_pocket_item(key: String, access_token: String, item: Item, source_name: String) {
    let mut map = HashMap::new();
    map.insert("consumer_key", key);
    map.insert("access_token", access_token);
    map.insert("url", item.link);
    map.insert("title", item.title);

    let mut categories = vec!();
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

    map.insert("tags", categories.join(","));

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

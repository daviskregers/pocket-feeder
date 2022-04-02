use crate::PocketItem;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use crate::pocket_item_response::PocketItemResponse;
    use crate::pocket_item::PocketItem;
    use std::collections::HashMap;

    #[test]
    fn pocket_item_response_has_link_returns_false_when_no_items() {
        let response = PocketItemResponse {
            status: 1,
            complete: 1,
            list: HashMap::new(),
        };

        assert!(response.has_link("home_link") == false);
    }

    #[test]
    fn pocket_item_response_has_link_returns_false_when_doesnt_match() {
        let mut list = HashMap::new();
        let item = PocketItem {
            given_title: "some title".to_string(),
            given_url: "some url".to_string(),
        };
        list.insert("some_id".to_string(), item);

        let response = PocketItemResponse {
            status: 1,
            complete: 1,
            list: list,
        };

        assert!(response.has_link("home_link") == false);
    }

    #[test]
    fn pocket_item_response_has_link_returns_true() {
        let mut list = HashMap::new();
        let item = PocketItem {
            given_title: "some title".to_string(),
            given_url: "some url".to_string(),
        };
        list.insert("some_id".to_string(), item);

        let response = PocketItemResponse {
            status: 1,
            complete: 1,
            list: list,
        };

        assert!(response.has_link("some url") == true);
    }

}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PocketItemResponse {
    pub status: u8,
    pub complete: u8,
    pub list: HashMap<String, PocketItem>,
}

impl PocketItemResponse {
    pub fn has_link(self: Self, link: &str) -> bool {
        for (_k, v) in self.list {
            if v.given_url == link {
                return true;
            }
        }
        return false;
    }
}

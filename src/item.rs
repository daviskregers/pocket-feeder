use serde::{Deserialize, Serialize};
use chrono::DateTime;

#[cfg(test)]
mod tests {
    use crate::item::Item;

    #[test]
    fn item_adds_timestamps() {
        let mut item = Item {
            author: Some(vec!["Dave".to_string()]),
            category: None,
            description: "Some article".to_string(),
            link: "http://example.org/article".to_string(),
            pubDate: "Mon, 28 Mar 2022 18:04:31 +0000".to_string(),
            timestamp: None,
            title: "A very good title".to_string(),
            utc: None,
        };

        item.add_timestamps();

        assert_eq!(item.utc, Some("2022-03-28 18:04:31".to_string()));
        assert_eq!(item.timestamp, Some(1648490671));
    }

    #[test]
    #[should_panic(expected = "Error while parsing pubDate: ParseError(Invalid)")]
    fn item_add_timestamp_throws_when_incorrect_pub_date() {
        let mut item = Item {
            author: Some(vec!["Dave".to_string()]),
            category: None,
            description: "Some article".to_string(),
            link: "http://example.org/article".to_string(),
            pubDate: "something not correct".to_string(),
            timestamp: None,
            title: "A very good title".to_string(),
            utc: None,
        };

        item.add_timestamps()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[allow(non_snake_case)]
pub struct Item {
    pub author: Option<Vec<String>>,
    pub category: Option<Vec<String>>,
    pub description: String,
    pub link: String,
    pub pubDate: String,
    pub timestamp: Option<i64>,
    pub utc: Option<String>,
    pub title: String,
}

impl Item {
    pub fn add_timestamps(self: &mut Self) {
        let datetime = DateTime::parse_from_rfc2822(self.pubDate.as_str())
            .expect("Error while parsing pubDate")
            .naive_utc();

        self.timestamp = Some(datetime.timestamp());
        self.utc = Some(datetime.to_string());
    }
}

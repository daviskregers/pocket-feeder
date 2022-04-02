use crate::source::Source;
use serde::{Deserialize, Serialize};
use std::fs;

#[cfg(test)]
mod tests {
    use tempfile::tempdir;
    use std::fs::File;
    use std::io::Write;
    use crate::source::{Source, ExcludeRules};
    use crate::config::{Config, PocketConfiguration, SentryConfiguration};
    use crate::config::get_config;

    #[test]
    fn config_reads_config_file() {
        let config = Config {
            pocket: PocketConfiguration {
                consumer: "some token".to_string(),
            },
            sentry: Some(SentryConfiguration {
                dsn: Some("some dsn".to_string()),
            }),
            sources: vec![
                Source {
                    src: "http://example.org".to_string(),
                    name: "example".to_string(),
                    exclude: None,
                },
                Source {
                    src: "http://example.org".to_string(),
                    name: "example".to_string(),
                    exclude: Some(
                        ExcludeRules {
                            author: Some(vec!["author 1".to_string(), "author 2".to_string()]),
                            category: Some(vec!["category 1".to_string(), "category 2".to_string()]),
                        }
                    ),
                }
            ]
        };

        let dir = tempdir().expect("Could not open a tempdir");
        let path = dir.path().join("get_config.yml").as_path().display().to_string();
        let mut file = File::create(&path).expect("Could not create a tempfile");
        let serialized = serde_yaml::to_string(&config).expect("Could not serialize test config");
        write!(file, "{}", serialized).expect("Failed to write into tempfile");

        let actual = get_config(&path);
        let actual_serialized = serde_yaml::to_string(&actual).expect("Could not serialize actual value");

        assert_eq!(serialized, actual_serialized);
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub pocket: PocketConfiguration,
    pub sentry: Option<SentryConfiguration>,
    pub sources: Vec<Source>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PocketConfiguration {
    pub consumer: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SentryConfiguration {
    pub dsn: Option<String>,
}

pub fn get_config(path: &str) -> Config {
    println!("Reading file: {}", path);
    let config_text: String = fs::read_to_string(path).expect("Something went wrong when reading the file: {}");
    serde_yaml::from_str(&config_text).expect("Something went wrong when parsing sources.yaml")
}


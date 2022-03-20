use std::env;
use std::fs;
use serde::Deserialize;


extern crate serde_yaml;

#[derive(Deserialize, Debug)]
struct Source {
    src: String
}

#[derive(Deserialize, Debug)]
struct SourceList {
    sources: Vec<Source>
}

fn main() {
    let file : String = env::current_dir().unwrap().display().to_string() + "/sources.yml";
    println!("Reading file: {}", file);

    let sources_text : String = fs::read_to_string(file)
            .expect("Something went wrong when reading the file: {}");

    let sources : SourceList = serde_yaml::from_str(&sources_text)
                .expect("Something went wrong when parsing sources.yaml");

    for source in &sources.sources {
        println!("Got source: {:?}", source);
    }
}

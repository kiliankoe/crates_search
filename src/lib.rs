extern crate reqwest;
extern crate json;

use std::io::Read;
use json::JsonValue;

pub enum Error {
    Network(reqwest::Error),
    Json(json::Error),
}

pub struct Crate {
    pub name: String,
    pub version: String,
    pub description: String,
    pub downloads: u32,
    pub license: String,
    pub homepage_url: String,
    pub repository_url: String,
    pub documentation_url: String,
}

pub fn search(query: &str) {
    let url = format!("https://crates.io/api/v1/crates?page=1&per_page=100&q={}",
                      query);
    let mut res = reqwest::get(&url).unwrap();

    let mut response = String::new();
    let _ = res.read_to_string(&mut response);

    let parsed = json::parse(&response).unwrap();

    println!("{}", parsed);
}

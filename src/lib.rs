extern crate reqwest;
extern crate json;

use std::io::Read;
use json::JsonValue;

#[derive(Debug)]
pub enum Error {
    Network(reqwest::Error),
    Json(json::Error),
    Decoding(&'static str),
}

#[derive(Debug)]
pub struct Crate {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub license: Option<String>,
    pub homepage_url: Option<String>,
    pub repository_url: Option<String>,
    pub documentation_url: Option<String>,
}

impl Crate {
    pub fn from_json(json: &JsonValue) -> Result<Crate, Error> {
        let krate = Crate {
            name: json_string(&json, "name")?,
            version: json_string(&json, "max_version")?,
            description: json_option_string(&json, "description"),
            license: json_option_string(&json, "license"),
            homepage_url: json_option_string(&json, "homepage"),
            repository_url: json_option_string(&json, "repository"),
            documentation_url: json_option_string(&json, "documentation"),
        };

        Ok(krate)
    }
}

fn json_option_string(json: &JsonValue, name: &'static str) -> Option<String> {
    let str = match json[name].as_str() {
        Some(str) => str,
        None => return None,
    };
    Some(String::from(str))
}

fn json_string(json: &JsonValue, name: &'static str) -> Result<String, Error> {
    // not sure if this is a stupid idea or not
    let str = json[name].as_str().ok_or_else(|| Error::Decoding(name))?;
    Ok(String::from(str))
}

pub fn search(query: &str) -> Result<Vec<Crate>, Error> {
    let url = format!("https://crates.io/api/v1/crates?page=1&per_page=100&q={}",
                      query);
    let mut res = reqwest::get(&url).unwrap();

    let mut response = String::new();
    let _ = res.read_to_string(&mut response);

    let json = match json::parse(&response) {
        Ok(val) => val,
        Err(err) => return Err(Error::Json(err)),
    };

    let json = match json["crates"] {
        JsonValue::Array(ref arr) => arr,
        _ => return Err(Error::Decoding("Found non-array-value in `crates`.")),
    };

    let mut crates: Vec<Crate> = Vec::new();
    for json_val in json {
        let krate = Crate::from_json(&json_val)?;
        crates.push(krate);
    }

    Ok(crates)
}

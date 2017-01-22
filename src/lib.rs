#[macro_use]
extern crate serde_derive;

extern crate reqwest;
extern crate serde_json;

#[derive(Deserialize, Debug)]
pub struct Crate {
    pub name: String,
    #[serde(rename = "max_version")]
    pub version: String,
    pub description: Option<String>,
    pub license: Option<String>,
    #[serde(rename = "homepage")]
    pub homepage_url: Option<String>,
    #[serde(rename = "repository")]
    pub repository_url: Option<String>,
    #[serde(rename = "documentation")]
    pub documentation_url: Option<String>,
}

/// Wrapper used by crates.io API.
#[derive(Deserialize)]
struct Crates {
    crates: Vec<Crate>,
}

pub fn search(query: &str) -> Result<Vec<Crate>, reqwest::Error> {
    let url = format!("https://crates.io/api/v1/crates?page=1&per_page=100&q={}",
                      query);

    let client = reqwest::Client::new().expect("Failed to create new reqwest client");
    // Working around a possible bug in reqwest/hyper on macOS, see https://github.com/seanmonstar/reqwest/issues/26
    let mut headers = reqwest::header::Headers::new();
    headers.set(reqwest::header::Connection::close());

    client.get(&url)
        .headers(headers)
        .send()?
        .json()
        .map(|response: Crates| response.crates)
}

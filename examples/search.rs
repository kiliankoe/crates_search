extern crate crates_search;

use crates_search::*;

fn main() {
    let crates = search("reqwest").expect("found no crates with specified query");
    println!("{:?}", crates);
}

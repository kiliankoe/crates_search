# 📦 crates_search

Search for crates on [crates.io](https://crates.io) from within Rust with this tiny wrapper.

This was quickly thrown together and will probably not satisfy any use case besides [my own](https://github.com/kiliankoe/alfred_crates). Also please be aware that it was written by an utter imbecile and Rust-novice (me 🙋‍♂️).

### Example

```rust
extern crate crates_search;

fn main() {
    let crates = crates_search::search("crates_search").unwrap();
    println!("{:?}", crates);
}
```

# rust-anilist 🚀

Just an [Anilist](https://anilist.co/) API wrapper made in Rust.

## Features

- Basic functionality to interact with the Anilist API.
- Asynchronous methods to load full details of entities like Anime, Manga, User, Person, and Character.
- Comprehensive data models with detailed documentation.

## Current status

It's working, just the basics, but it already works.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
rust-anilist = "*"
```

## Usage

Here's a basic example of how to use the library:

```rust
use rust_anilist::Client;

#[tokio::main]
async fn main() {
    let client = Client::with_token("your_api_key");
    let anime = client.get_anime(1).await.unwrap();
    println!("{:?}", anime);
}
```

## Documentation

The library is fully documented. You can find the documentation [here](https://docs.rs/rust-anilist).

## License

Copyright © 2022-2025 [AndrielFR](https://github.com/AndrielFR)

Licensed under the [Expat/MIT license](LICENSE).
This project is also [REUSE compliant](https://reuse.software/).
See individual files for more copyright information.

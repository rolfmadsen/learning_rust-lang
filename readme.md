# Rust playground

Install rust:
```console
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Update rustup:
```console
rustup update
```

Check version:
```console
rustc --version
```

Install C compiler:
```console
sudo apt install build-essential
```

## Build project

```console
cargo new [PROJECT-NAME]
```

## Install Cargo dependencies

```console
cargo fetch
```
NB. Similar to npm install

### Compile main.rs
```console
cargo build --dev
```
--

### Compile and run main.rs
```console
    cargo run --release
```

## Rust & Serde

### Handling missing JSON fields

    #[serde(skip_serializing_if = "Option::is_none")]

(Error handling - Option & unwrap - Unpacking options with ?)[https://doc.rust-lang.org/rust-by-example/error/option_unwrap/question_mark.html]

(Rust: Using Options by example)[https://www.ameyalokare.com/rust/2017/10/23/rust-options.html]
(Rust Arrays Tutorial)[https://www.koderhq.com/tutorial/rust/array/]
(Deserialize variable meta object with serde)[https://stackoverflow.com/a/61646314]

(Vec & Indexing)[https://doc.rust-lang.org/beta/std/vec/struct.Vec.html#indexing]
(Deserialize a JSON string or array of strings into a Vec)[https://www.javaer101.com/en/article/15848189.html]
(From Go to Rust: JSON and YAML)[https://dzone.com/articles/from-go-to-rust-json-and-yaml]

#### Option

(The Option Enum and Its Advantages Over Null Values)[https://doc.rust-lang.org/stable/book/ch06-01-defining-an-enum.html]

### Rust books

(Cheat sheets & books)[https://cheats.rs/]
(The Rust Programming Language)[file:///home/INSERT-NAME/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/share/doc/rust/html/book/index.html]

## Auto genereate structs from JSON

(https://app.quicktype.io/)[https://app.quicktype.io/]

## Tutorials

https://www.educative.io/courses/learn-rust-from-scratch/
https://www.educative.io/courses/ultimate-guide-to-rust-programming


## Backup ressources

// https://www.reddit.com/r/rust/comments/7hasv6/mixed_valuestruct_deserialization_with_serde_json/dqpht6v?utm_source=share&utm_medium=web2x&context=3
// https://transform.tools/json-to-rust-serde

// Unwrapping Rust's errors - https://medium.com/swlh/unwrapping-rusts-errors-552e583e2963

Serde - Generiske parametre:
https://doc.rust-lang.org/book/ch10-01-syntax.html
https://doc.rust-lang.org/beta/rust-by-example/generics/assoc_items/types.html
https://serde.rs/
https://github.com/dtolnay/serde-yaml/issues/195
https://github.com/serde-rs/serde/issues/984

https://learning-rust.github.io/docs/e3.option_and_result.html#is-some-is-none-is-ok-is-err

https://www.softax.pl/blog/rust-lang-in-a-nutshell-2-enums-pattern-matching-options/

## Initialize HashMap

Tutorial: https://www.koderhq.com/tutorial/rust/hashmap/

```rust

use std::collections::HashMap;

fn main() {

let mut language: HashMap<&str, &str> = HashMap::new();

language.insert("Einar", "Norway");
language.insert("Olaf", "Denmark");
language.insert("Harald", "Iceland");

println!("{:?}", language);

}

```

## Struct and HashMap

```rust
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    email: Option<String>,
    name: Option<String>,
}

impl User {
    fn new(attributes: &HashMap<String, String>) -> User {
        User {
            email: attributes.get("email").cloned(),
            name: attributes.get("name").cloned(),
        }
    }
}

fn main() {
    let mut map = HashMap::new();
    map.insert(String::from("email"), String::from("foo@bar.de"));
    map.insert(String::from("name"), String::from("John Doe"));

    let user_model = User::new(&map);
    let p = serde_json::to_string(&user_mode).unwrap();

    println!("{}", p);
}

```
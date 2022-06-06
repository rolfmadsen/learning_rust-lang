use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Deserialize, Debug)]
pub struct Query {
    query_value: String,
}

pub fn read_query_from_file<P: AsRef<Path>>(path: P) -> Result<Query, Box<dyn Error>> {
    let file = File::open(path).expect("Failed to open query.json");   // Open the file in read-only mode with buffer.
    let reader = BufReader::new(file);    
    let u = serde_json::from_reader(reader)?;   // Read the JSON contents of the file as an instance of `Query`.
    Ok(u)   // Return the `Query`.
}

pub fn query_string() -> std::string::String {
    let u = read_query_from_file("src/query.json").unwrap();
    u.query_value
}

// Source: https://docs.serde.rs/serde_json/fn.from_reader.html
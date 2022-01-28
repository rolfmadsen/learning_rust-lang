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
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let u = serde_json::from_reader(reader)?;

    // Return the `User`.
    Ok(u)
}

pub fn query_string() -> std::string::String {
    let u = read_query_from_file("query.json").unwrap();
    let return_value = format!("{}", u.query_value); // NB. Use "let return_value = format!("{:}", u.query_value);" to escape single quotes in search query.
    //println!("{:?}", return_value);
    return return_value
}

/* Used for test purposes:
fn main() {
    let print_value = query_string();
    println!("{:?}", print_value)
}
*/
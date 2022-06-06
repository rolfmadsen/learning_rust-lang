use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use serde_json;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Query {
    pub query_value: String,
}

pub fn query_string() {
    let path = "/query.json";
    let data = fs::read_to_string(path).expect("Missing query string!");
    let query = serde_json::from_str(&data);
    let query_string = Query { query_value };
    let formatted_query_string = format!("{:#?}", query_string);
}
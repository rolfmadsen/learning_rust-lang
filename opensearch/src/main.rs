#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use reqwest::Error;
mod query;
//use serde_json;
//use std::collections::HashMap;
//use url;
//use std::convert::From;

use opensearch::OpenSearch;
#[path="./structs/opensearch.rs"]
pub mod opensearch;

// https://opensearch.addi.dk/test_5.2/?action=search&query=%22peter%20hansen%22&agency=100200&profile=test&start=1&stepValue=5&outputType=json

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Bibob {
    pub bibob_hitcount: i32,
}


#[tokio::main]
async fn main() -> Result<(), Error> {

    let query_string = query::query_string();
    //"phrase.creator='peter hansen'";

    let request_url = format!("https://opensearch.addi.dk/{version}/?action={action}&query={query}&agency={agency}&profile={profile}&start={start}&stepValue={step_value}&outputType={output_type}",
        version = "test_5.2", //b3.5_5.2
        action = "search",
        query = &query_string,
        agency = "100200",
        profile = "test",
        start = "1",
        step_value = "50",
        output_type = "json"
    );
    // Perform the actual execution of the network request
    println!("{}", request_url);
    let service_response = reqwest::get(&request_url).await?;
    let open_search: OpenSearch = service_response.json().await?;

    let hit_count_value: i32 = open_search.search_response.result.hit_count.field.parse::<i32>().unwrap();
    if hit_count_value > 0 {
        println!("Your search for query gave {:?} results!", &hit_count_value);

        let bibob_hitcount = hit_count_value;
        let format_hitcount = Bibob { bibob_hitcount };
        println!("{:?}", format_hitcount);
        let format_hitcount_json = serde_json::to_string(&format_hitcount).unwrap();
        println!("{}", format_hitcount_json);
        
        let path_to_search_result = &open_search.search_response.result.search_result;
        match path_to_search_result {
            Some(path_to_search_result) => {
                for collection in path_to_search_result.iter() {
                    let path_to_collection = &collection.collection.object;
                    for object in path_to_collection.iter() {
                        let record = &object.record;
                        let title = &record.title;
                        match title {
                            Some(i) => {
                                print!("Title: {:}", i[0].field);

                                let creator = &record.creator;
                                match creator {
                                    Some(i) => println!(" by creator: {:}", i[0].field),
                                    None => {
                                        let contributor = &record.contributor;
                                        match contributor {
                                            Some(i) => println!(" by contributor: {:}", i[0].field),
                                            None => {
                                                let publisher = &record.publisher;
                                                match publisher {
                                                    Some(i) => println!(" by publisher: {:}", i[0].field),
                                                    None => println!(" by unknown creator"),
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            None => println!("Missing title")
                        }
                    }
                }
            }
            None => println!("No search result!"),
        }
        
    } else {
        println!("Your search query gave {:?} results!", hit_count_value);
    }
    Ok(())
}
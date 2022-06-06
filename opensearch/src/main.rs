#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use reqwest::Error;
use url;
mod query;
use std::convert::From;

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



#[derive(Default, Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenSearch {
    pub search_response: SearchResponse,
}

#[derive(Default, Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResponse {
    pub result: SearchResponseResult,
}

#[derive(Default, Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResponseResult {
    pub hit_count: HitCount,
    pub collection_count: CollectionCount,
    pub more: More,
    #[serde(rename = "searchResult")]
    pub search_result: Option<Vec<SearchResult>>,
    #[serde(rename = "facetResult")]
    pub facet_result: FacetResult,
}

#[derive(Default, Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HitCount {
    #[serde(rename = "$")]
    pub field: String,
}

#[derive(Default, Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionCount {
    #[serde(rename = "$")]
    pub field: String,
}

#[derive(Default, Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct More {
    #[serde(rename = "$")]
    pub field: String,
}

#[derive(Default, Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    pub collection: Collection,
}

#[derive(Default, Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    pub result_position: ResultPosition,
    pub number_of_objects: NumberOfObjects,
    pub object: Vec<Object>,
}

#[derive(Default, Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResultPosition {
    #[serde(rename = "$")]
    pub field: String,
}

#[derive(Default, Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NumberOfObjects {
    #[serde(rename = "$")]
    pub field: String,
}

#[derive(Default, Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Object {
    pub record: Record,
    pub identifier: Identifier2,
    pub primary_object_identifier: PrimaryObjectIdentifier,
    pub record_status: RecordStatus,
    pub creation_date: CreationDate,
    pub formats_available: FormatsAvailable,
    pub objects_available: ObjectsAvailable,
}

#[derive(Default, Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Record {
    pub identifier: Option<Vec<Identifier>>,
    pub source: Option<Vec<Source>>,
    pub title: Option<Vec<Title>>,
    pub creator: Option<Vec<Creator>>,
    pub subject: Option<Vec<Subject>>,
    #[serde(default)]
    pub description: Option<Vec<Description>>,
    pub audience: Option<Vec<Audience>>,
    #[serde(default)]
    pub publisher: Option<Vec<Publisher>>,
    #[serde(default)]
    pub contributor: Option<Vec<Contributor>>,
    pub date: Option<Vec<Date>>,
    #[serde(rename = "type")]
    pub type_field: Option<Vec<Type>>,
    #[serde(default)]
    pub format: Option<Vec<Format>>,
    #[serde(default)]
    pub extent: Option<Vec<Extent>>,
    pub language: Option<Vec<Language>>,
    #[serde(default)]
    pub temporal: Option<Vec<Temporal>>,
    #[serde(rename = "@")]
    pub field: String,
    #[serde(default)]
    pub spatial: Option<Vec<Spatial>>,
    #[serde(default)]
    pub version: Option<Vec<Version>>,
    #[serde(default)]
    pub alternative: Option<Vec<Alternative>>,
    #[serde(default)]
    pub shelf: Option<Vec<Shelf>>,
    #[serde(default)]
    pub has_part: Option<Vec<HasPart>>,
    #[serde(rename = "abstract")]
    #[serde(default)]
    pub abstract_field: Option<Vec<Abstract>>,
    #[serde(default)]
    pub is_part_of: Option<Vec<IsPartOf>>,
}

#[derive(Default, Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Identifier {
    #[serde(rename = "$")]
    pub field: String,
    #[serde(rename = "@")]
    pub field2: String,
    #[serde(rename = "@type")]
    pub type_field: Option<Box<Type>>,
}

#[derive(Default, Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Type {   
    #[serde(rename = "$")]
    pub type_value: String,
    #[serde(rename = "@type")]	
    pub type_prefix: Option<Box<Type>>,    
    #[serde(rename = "@")]
    pub type_namespace: String,
}

#[derive(Default, Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaterialType {   
    #[serde(rename = "$")]
    pub value: String,
    #[serde(rename = "@type")]	
    pub prefix: Option<Box<Type>>,   
    #[serde(rename = "@")]
    pub namespace: String,
}

#[derive(Default, Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    #[serde(rename = "$")]
    pub field: String,
    #[serde(rename = "@")]
    pub field2: String,
}

#[derive(Default, Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Title {
    #[serde(rename = "$")]
    pub field: String,
    #[serde(rename = "@")]
    pub field2: String,
    #[serde(rename = "@type")]
    pub type_field: Option<Box<Type>>,
}

#[derive(Default, Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Creator {
    #[serde(rename = "$")]
    pub field: String,
    #[serde(rename = "@type")]
    pub type_field: Option<Box<Type>>,
    #[serde(rename = "@")]
    pub field2: String,
}

#[derive(Default, Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subject {
    #[serde(rename = "$")]
    pub field: String,
    #[serde(rename = "@type")]
    pub type_field: Option<Box<Type>>,
    #[serde(rename = "@")]
    pub field2: String,
}

#[derive(Default, Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Description {
    #[serde(rename = "$")]
    pub field: String,
    #[serde(rename = "@")]
    pub field2: String,
}

#[derive(Default, Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Audience {
    #[serde(rename = "$")]
    pub field: String,
    #[serde(rename = "@")]
    pub field2: String,
}

#[derive(Default, Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Publisher {
    #[serde(rename = "$")]
    pub field: String,
    #[serde(rename = "@")]
    pub field2: String,
}

#[derive(Default, Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contributor {
    #[serde(rename = "$")]
    pub field: String,
    #[serde(rename = "@")]
    pub field2: String,
    #[serde(rename = "@type")]
    pub type_field: Option<Box<Type>>,
}

#[derive(Default, Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Date {
    #[serde(rename = "$")]
    pub field: String,
    #[serde(rename = "@")]
    pub field2: String,
}

#[derive(Default, Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Format {
    #[serde(rename = "$")]
    pub field: String,
    #[serde(rename = "@")]
    pub field2: String,
}

#[derive(Default, Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Extent {
    #[serde(rename = "$")]
    pub field: String,
    #[serde(rename = "@")]
    pub field2: String,
}

#[derive(Default, Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Language {
    #[serde(rename = "$")]
    pub field: String,
    #[serde(rename = "@type")]
    pub type_field: Option<Box<Type>>,
    #[serde(rename = "@")]
    pub field2: String,
}

#[derive(Default, Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Temporal {
    #[serde(rename = "$")]
    pub field: String,
    #[serde(rename = "@type")]
    pub type_field: Option<Box<Type>>,
    #[serde(rename = "@")]
    pub field2: String,
}

#[derive(Default, Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Spatial {
    #[serde(rename = "$")]
    pub field: String,
    #[serde(rename = "@type")]
    pub type_field: Option<Box<Type>>,
    #[serde(rename = "@")]
    pub field2: String,
}

#[derive(Default, Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    #[serde(rename = "$")]
    pub field: String,
    #[serde(rename = "@")]
    pub field2: String,
}

#[derive(Default, Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Alternative {
    #[serde(rename = "$")]
    pub field: String,
    #[serde(rename = "@")]
    pub field2: String,
}

#[derive(Default, Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Shelf {
    #[serde(rename = "$")]
    pub field: String,
    #[serde(rename = "@type")]
    pub type_field: Option<Box<Type>>,
    #[serde(rename = "@")]
    pub field2: String,
}


#[derive(Default, Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HasPart {
    #[serde(rename = "$")]
    pub field: String,
    #[serde(rename = "@type")]
    pub type_field: Option<Box<Type>>,
    #[serde(rename = "@")]
    pub field2: String,
}

#[derive(Default, Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Abstract {
    #[serde(rename = "$")]
    pub field: String,
    #[serde(rename = "@")]
    pub field2: String,
}

#[derive(Default, Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IsPartOf {
    #[serde(rename = "$")]
    pub field: String,
    #[serde(rename = "@")]
    pub field2: String,
}

#[derive(Default, Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Identifier2 {
    #[serde(rename = "$")]
    pub field: String,
}

#[derive(Default, Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrimaryObjectIdentifier {
    #[serde(rename = "$")]
    pub field: String,
}

#[derive(Default, Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordStatus {
    #[serde(rename = "$")]
    pub field: String,
}

#[derive(Default, Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreationDate {
    #[serde(rename = "$")]
    pub field: String,
}

#[derive(Default, Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormatsAvailable {
    pub format: Option<Vec<Format2>>,
}

#[derive(Default, Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Format2 {
    #[serde(rename = "$")]
    pub field: String,
}

#[derive(Default, Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectsAvailable {
    pub identifier: Option<Vec<Identifier3>>,
}

#[derive(Default, Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Identifier3 {
    #[serde(rename = "$")]
    pub field: String,
}

#[derive(Default, Debug, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FacetResult {
    #[serde(rename = "$")]
    pub field: String,
}
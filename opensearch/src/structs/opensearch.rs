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
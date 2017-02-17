use models::misc::{Pagination, Type};
use std::collections::HashMap;


#[derive(Deserialize, Debug)]
pub struct SearchResult {
    pagination: Pagination,
    results: Vec<SearchResultItem>,
}


#[derive(Deserialize, Debug)]
pub struct SearchResultItem {
    id: i64,
    #[serde(rename = "type")]
    item_type: Type,
    style: Vec<String>,
    thumb: String,
    title: String,
    country: String,
    format: Vec<String>,
    uri: String,
    community: HashMap<String, i32>,
    label: Vec<String>,
    catno: String,
    year: Option<String>,
    genre: Vec<String>,
    resource_url: String,
}

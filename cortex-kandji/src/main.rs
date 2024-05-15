use std::env;
use reqwest::{Client};
use serde::{Serialize,Deserialize};
use serde_json;

struct CortexEndpoint{
    hostname:String,
    agent_id: String
}

#[derive(Serialize,Deserialize)]
struct SortField{
    field: String,
    keyword: String
}

#[derive(Serialize,Deserialize)]
struct Filter{
    field: String,
    operator: String,
    value: Vec<String>
}

#[derive(Serialize,Deserialize)]
struct CortexRequestBody{
    search_from : i32,
    search_to : i32,
    sort: SortField,
    filters: Vec<Filter>
}


fn get_cortex_endpoints() {
    let mut start = 0;
    let mut end = 100;
    let url = String::from(env::var("CORTEX_URL"));
    let cortex_api_key = String::from(env::var("CORTEX_API_KEY"));
    let client = reqwest::Client::new();
    
    let payload = CortexRequestBody{
        search_from: start,
        search_to: end,
        sort: SortField{field:"endpoint_id",keyword:"asc"},
        filters:vec![Filter{
            field:"endpoint_status",
            operator:"in",
            value:vec!["connected","disconneted"]
        }]
    };

    let mut headers = HeaderMap::new();
    

}
fn main() {
    println!("Hello, world!");
}


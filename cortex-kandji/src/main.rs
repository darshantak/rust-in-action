use std::env;
use reqwest::{header::HeaderMap, Client};
use serde::{Serialize,Deserialize};
use serde_json::{self, error};

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

#[allow(unused)]
fn get_cortex_endpoints() -> Result<String,std::error::Error> {
    let mut start = 0;
    let mut end = 100;
    let url = env::var("CORTEX_URL").expect("Cortex URL not found.");
    let cortex_api_key = env::var("CORTEX_API_KEY").expect("Cortex API_KEY not found");
    let client = reqwest::blocking::Client::new();
    
    let payload = CortexRequestBody{
        search_from: start,
        search_to: end,
        sort: SortField{field:"endpoint_id".to_string(),keyword:"asc".to_string()},
        filters:vec![Filter{
            field:"endpoint_status".to_string(),
            operator:"in".to_string(),
            value:vec!["connected".to_string(),"disconneted".to_string()]
        }]
    };
    let mut headers = HeaderMap::new();
    let payload_json = serde_json::to_string(&payload)?;

    let response = client.post(url).body(payload_json).send();


}
fn main() {
    println!("Hello, world!");
}


use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::{env, fmt::Display, io};

struct CortexEndpoint {
    hostname: String,
    agent_id: String,
}

#[derive(Serialize, Deserialize)]
struct SortField {
    field: String,
    keyword: String,
}

#[derive(Serialize, Deserialize)]
struct Filter {
    field: String,
    operator: String,
    value: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct CortexRequestBody {
    search_from: i32,
    search_to: i32,
    sort: SortField,
    filters: Vec<Filter>,
}

#[derive(Debug)]
enum ErrorTypes {
    Io(io::Error),
    Serde(serde_json::Error),
    Reqwest(reqwest::Error),
}

impl Display for ErrorTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ErrorTypes::Io(ref err) => {
                write!(f, "IO error with {}", err)
            }
            ErrorTypes::Reqwest(ref err) => {
                write!(f, "Reqwest Error with {}", err)
            }
            ErrorTypes::Serde(ref err) => {
                write!(f, "Serde error with {}", err)
            }
        }
    }
}

impl Error for ErrorTypes {}

impl From<io::Error> for ErrorTypes {
    fn from(value: io::Error) -> Self {
        ErrorTypes::Io(value)
    }
}

impl From<reqwest::Error> for ErrorTypes {
    fn from(value: reqwest::Error) -> Self {
        ErrorTypes::Reqwest(value)
    }
}

impl From<serde_json::Error> for ErrorTypes {
    fn from(value: serde_json::Error) -> Self {
        ErrorTypes::Serde(value)
    }
}

#[allow(unused)]
fn get_cortex_endpoints() -> Result<(), Box<dyn Error>> {
    let mut start = 0;
    let mut end = 100;
    let url = env::var("CORTEX_URL").expect("Cortex URL not found.");
    let cortex_api_key = env::var("CORTEX_API_KEY").expect("Cortex API_KEY not found");
    let client = reqwest::blocking::Client::new();

    let payload = CortexRequestBody {
        search_from: start,
        search_to: end,
        sort: SortField {
            field: "endpoint_id".to_string(),
            keyword: "asc".to_string(),
        },
        filters: vec![Filter {
            field: "endpoint_status".to_string(),
            operator: "in".to_string(),
            value: vec!["connected".to_string(), "disconnected".to_string()],
        }],
    };
    let mut headers = HeaderMap::new();
    // let de_cortex_api_key = HeaderValue::from_str(&cortex_api_key)?;
    headers.append("x-xdr-auth-id", HeaderValue::from_static("7"));
    headers.append("Authorization", HeaderValue::from_str(&cortex_api_key)?);
    // headers.append("Authorization", de_cortex_api_key);
    headers.append("Content-Type", HeaderValue::from_static("application/json"));

    let response = client.post(url).headers(headers).json(&payload).send()?;
    let response_text:serde_json::Value = response.json()?;
    println!("{:?}",response_text);
    Ok(())
}

fn main() {
    if let Err(err) = get_cortex_endpoints(){
        println!("{:?}",err)
    }

    
}

use std::env;
use hyper:: { Body, Client, Response, Request };
use hyper::client::connect::HttpConnector;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use tokio;
use float_ord::{FloatOrd};

#[derive(Serialize, Deserialize)]
struct MenuItem {
    name: String,
    price: f64,
}

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
struct Menu {
    items: Vec<MenuItem>,
}


async fn get(url: String) -> String {
    // Get the json
    println!("Fetching from {url}.");

    let http = HttpConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(http);

    let res = client.request(
        Request::builder()
        .method("GET")
        .uri(url.clone())
        .body(Body::from("".to_string()))
        .unwrap()
    ).await;

    match res {
        Err(_) => panic!("Failed to GET from {url}"),
        Ok(r) => {
            return std::str::from_utf8(&hyper::body::to_bytes(r.into_body()).await.unwrap()).unwrap().to_string().clone();
        },
    }
}

#[tokio::main]
async fn main() -> () {
    // Get the url
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("You must have one argument: the url of the api endpoint");
        return;
    }

    let url: String = args[1].clone();
    let json: String = get(url.clone()).await;

    // Parse the json
    println!("Parsing json.");
    let res = serde_json::from_str(json.as_str());
    let mut menu: Menu;
    match res {
        Err(_) => panic!("Cannot parse json"),
        Ok(m) => menu = m,
    }

    // Process output
    _ = &menu.items.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());
    for item in menu.items.iter(){
        let name = &item.name;
        let price = &item.price;
        println!("| {price} | {name} |");
    }

    return ();
}


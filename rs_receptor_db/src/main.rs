use axum::{
        routing::{get, post},
        Router,
        Json,
};
use std::net::SocketAddr;
use reqwest;
use std::io::{stdout, Write};
use log::{info, warn, error};
use std::string;
use serde_json::{Result, Value};

use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(homepage))
        .route("/get_var", get(get_var));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Server running at http://{}", addr);
    stdout().flush().unwrap();
    // Spin up connection to postgres db running over docker

    _convert_gene_json_db().await;
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


// Calls the uniprot API using curl and then converts the resulting
// json into a format that either is a DB or can easily be written
// to a db
// NOTE: Will definitely need to filter by taxon_label == "Homo sapiens" after the fact
async fn _convert_gene_json_db() {
    // Call GO API using cURL
    // handling a failed call to the api???
    // Store data in a json value
    let url = "https://api.geneontology.org/api/bioentity/function/GO:0007166?category=gene&taxon_label=Homo%20sapiens&rows=1000000";
    let res = reqwest::get(url).await.unwrap().text().await.unwrap();
    // Use curl client to get url utf-8 encoded byte stream json response
    let json_value: Vec<Value> = serde_json::from_str(&res).unwrap();
    // Add data from stack local buffer as records to a postgres db
    for item in &json_value {
        if item["taxon_label"] == "Homo sapiens" {
        }
    };

    // Return a Result
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Must set the database  url");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

async fn homepage() -> &'static str {
    "This is the homepage for receptor_db"
}
async fn get_var() -> &'static str {
    "This is the page for get_var"
}

use axum::{
        routing::{get, post},
        Router,
        Json,
};
use std::net::SocketAddr;
use curl::easy::Easy;
use std::io::{stdout, Write};
use log::{info, warn, error};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(homepage))
        .route("/get_var", get(get_var));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Server running at http://{}", addr);
    stdout().write_all("blah blah balh \n".as_bytes()).unwrap();
    stdout().flush().unwrap();
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
    let mut easy = Easy::new();
    let url = "https://api.geneontology.org/api/bioentity/function/GO:0007166?category=gene&taxon_label=Homo%20sapiens&rows=1000000";
    easy.url(url).unwrap();
    // Use curl client to get url text json response
    easy.get(false).unwrap();
    easy.write_function(|data| {
        Ok(stdout().write(data).unwrap_or_else(|error| {
            error!("Problem writing the file: {:?}", error);
            1
        }))
    }).unwrap();
    easy.perform().unwrap();
    stdout().flush().unwrap();
//    println!("{:?}", data);

    // Get a json string from the api result

    // Reformat in a way that makes sense
    
    // Add data as records to a postgres db
    
    // Return a Result
}

async fn homepage() -> &'static str {
    "This is the homepage for receptor_db"
}
async fn get_var() -> &'static str {
    "This is the page for get_var"
}

mod db;

use aws_sdk_dynamodb::Client;
use axum::{Router, routing::post};
use tower_http::cors::{CorsLayer, Any};
use shared::{SongEntry};


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .pretty()
        .init();

    let client = db::init_client().await;
    if client.is_err() { return }
    let client = client.unwrap();

    // rest of the implementations 
    // populating the table 
    populate_table(client).await;

    // then set up listeners etc etc
}

#[allow(dead_code)]
async fn populate_table(
    client: Client
) {
    let songs = vec![
        SongEntry::new(
            "test", 
            "tester", 
            "this is a test entry", 
            "no MIDI", 
            "no MIDI description", 
            "I love testing ! ~ I love testing ~ !"
        )
    ];

    if db::register_songs(client, songs).await.is_err() { println!("Issue with registering songs") }
}

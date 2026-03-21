use std::{error::Error, time::Duration};
use shared::{SongEntry, SongRecommendation, SongRequest};

use aws_sdk_dynamodb::{self as dynamodb, operation::create_table::CreateTableError, types::TableStatus, types::AttributeValue}; 
use tokio::time::{sleep};
use tracing::{event, Level};

pub async fn init_client() -> Result<dynamodb::Client, Box< dyn std::error::Error>> { 
    let config = aws_config::load_from_env().await;
    let client = dynamodb::Client::new(&config);
    
    let create_result = client
        .create_table()
        .table_name("songStorage")
        .send()
        .await;
    
    match create_result {
        Ok(_) => {
            event!(Level::INFO, "Successfully created the 'songStorage' table.");
        }
        Err(err) => {
            let service_err = err.into_service_error();
            match service_err {
                CreateTableError::ResourceInUseException(_) => {
                    event!(Level::INFO, "Table 'songStorage' already exists.");
                }
                other_error => {
                    event!(Level::DEBUG, error=%other_error, "Failed to create 'songStorage' table");
                }
            }
        }
    }

    match table_active(&client, "songStorage").await {
        Ok(_) => {
            Ok(client)
        },
        Err(err) => {
            event!(Level::DEBUG, err);
            Err("There was an issue while initializing Client.".into())
        },
    }
}

pub async fn table_active(
    client: &dynamodb::Client,
    table_name : &str
) -> Result<(), Box<dyn std::error::Error>> {
    let max_attempts = 20;
    let delay_seconds = 2;
    let mut attempts = 0;

    event!(Level::INFO, "Waiting for table '{}' to become ACTIVE...", table_name);

    loop {
        let response = client
            .describe_table()
            .table_name(table_name)
            .send()
            .await?;

        if let Some(table) = response.table() 
            && let Some(status) = table.table_status() {
                match status {
                    TableStatus::Active => {
                        return Ok(());
                    }
                    TableStatus::Creating | TableStatus::Updating => {
                        continue;
                    }
                    _ => {
                        return Err(format!("Unexpected table status: {:?}", status).into());
                    }
                }
            }

        attempts += 1;
        if attempts >= max_attempts {
            return Err("Timed out waiting for table to become ACTIVE".into());
        }

        sleep (Duration::from_secs(delay_seconds)).await;
    }
}

#[allow(dead_code)]
pub async fn register_songs(
    client: dynamodb::Client, 
    songs: Vec<SongEntry>
) -> Result<(), Box<dyn Error>> {
    for song in songs {
        register_song(client.clone(), song).await?;
    }

    Ok(())
}

#[allow(dead_code)]
pub async fn register_song(
    client: dynamodb::Client,
    song: SongEntry
) -> Result<(), Box<dyn Error>> {
    event!(Level::INFO, "Adding '{}' by {}...", song.name, song.artist);

    // todo! make sure it doesn't give error if song is already registered into table
    
    client
        .put_item()
        .table_name("songStorage")
        .item("Name", AttributeValue::S(song.name))
        .item("Artist", AttributeValue::S(song.artist))
        .item("Song Description", AttributeValue::S(song.description))
        .item("MIDI", AttributeValue::S(song.midi))
        .item("MIDI Description", AttributeValue::S(song.midi_description))
        .item("Lyrics Description", AttributeValue::S(song.lyrics_description))
        .send()
        .await?;

    Ok(())
}

#[allow(dead_code)]
pub async fn update_song(
    client: dynamodb::Client,
    song: SongEntry
) -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[allow(dead_code)]
pub async fn get_song(
    client: dynamodb::Client,
    song: SongRequest
) -> Result<SongRecommendation, Box<dyn Error>> {
    Ok(SongRecommendation::new("tmp", "tmp"))
}
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SongRequest {
    pub name: String,
    pub artist: String,
    pub inputs: u8, // 0 = overall, 1 = instrumental, 2 = lyrics
}

impl SongRequest {
    pub fn new(
        name: &str,
        artist: &str,
        inputs: u8,
    ) -> SongRequest {
        SongRequest {
            name: name.to_string(),
            artist: artist.to_string(),
            inputs
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SongRecommendation {
    pub name: String, 
    pub artist: String, 
}

impl SongRecommendation {
    pub fn new(
        name: &str,
        artist: &str,
    ) -> SongRecommendation {
        SongRecommendation { 
            name: name.to_string(),
            artist: artist.to_string(), 
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SongEntry {
    pub name: String, 
    pub artist: String, 
    pub description: String, 
    pub midi: String, 
    pub midi_description: String, 
    pub lyrics_description: String,
}

impl SongEntry {
    pub fn new(
        name: &str,
        artist: &str,
        description: &str,
        midi: &str, 
        midi_description: &str,
        lyrics_description: &str,
    ) -> SongEntry {
        SongEntry { 
            name: name.to_string(),
            artist: artist.to_string(), 
            description: description.to_string(), 
            midi: midi.to_string(), 
            midi_description: midi_description.to_string(), 
            lyrics_description: lyrics_description.to_string(), 
        }
    }
}
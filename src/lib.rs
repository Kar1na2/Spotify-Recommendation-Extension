use wasm_bindgen::prelude::*;
use web_sys::console;
use serde::{Deserialize, Serialize};

// Macro for easy console.log from Rust
macro_rules! log {
    ($($t:tt)*) => (console::log_1(&format!($($t)*).into()))
}

/// Structured song data passed between JS and Rust
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SongInfo {
    pub(crate) track_name: String,
    pub(crate) artist_name: String,
    pub(crate) platform: String,   // "spotify" | "youtube"
}

#[wasm_bindgen] 
impl SongInfo {
    #[wasm_bindgen(constructor)]
    pub fn new(
        track_name: &str,
        artist_name: &str,
        platform: &str,
    ) -> SongInfo {
        SongInfo { 
            track_name: track_name.to_string(), 
            artist_name: artist_name.to_string(), 
            platform: platform.to_string() 
        }
    }
    
    #[wasm_bindgen(getter)]
    pub fn track_name(&self) -> String { self.track_name.clone() }
    
    #[wasm_bindgen(getter)]
    pub fn artist_name(&self) -> String { self.artist_name.clone() }

    #[wasm_bindgen(getter)]
    pub fn platform(&self) -> String { self.platform.clone() }
}
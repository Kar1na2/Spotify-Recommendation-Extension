use wasm_bindgen::prelude::*;
#[cfg(debug_assertions)]
use wasm_bindgen_test::__rt::console_error;
use web_sys::console;
use serde::{Deserialize, Serialize};
use serde_json;

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
}

#[wasm_bindgen] 
impl SongInfo {
    #[wasm_bindgen(constructor)]
    pub fn new(
        track_name: &str,
        artist_name: &str,
    ) -> SongInfo {
        SongInfo { 
            track_name: track_name.to_string(), 
            artist_name: artist_name.to_string(), 
        }
    }
    
    #[wasm_bindgen(getter)]
    pub fn track_name(&self) -> String { self.track_name.clone() }
    
    #[wasm_bindgen(getter)]
    pub fn artist_name(&self) -> String { self.artist_name.clone() }
}

#[wasm_bindgen]
pub fn process_song_data(
    track_name: &str,
    artist_name: &str,
) -> Result<SongInfo, JsValue> {
    let track_name = track_name.trim().to_string();
    let artist_name = artist_name.trim().to_string();

    if track_name.is_empty() {
        return Err(JsValue::from_str("track_name cannot be empty"));
    }
    if artist_name.is_empty() {
        return Err(JsValue::from_str("artist_name cannot be empty"));
    }

    log!(
        "[WASM] Processed: '{}' by '{}'",
        track_name, artist_name
    );

    Ok(SongInfo::new(&track_name, &artist_name))
}

#[wasm_bindgen]
pub fn song_to_json(song: &SongInfo) -> Result<String, JsValue> {
    serde_json::to_string(song)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen(start)] 
pub fn init() {
    log!("[WASM] wasm_core initialized");
}
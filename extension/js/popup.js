document.addEventListener('DOMContentLoaded', () => {
    console.log("popup.js DOM fully loaded");
    
    // 1. Initial load: Get whatever is currently playing
    currentSong();

    // 2. Listen for live updates from background.js
    chrome.runtime.onMessage.addListener((message) => {
        if (message.type === 'UPDATE_POPUP' && message.song) {
            displaySong(message.song);
        }
    });
});

async function currentSong() {
    const msg = await chrome.runtime.sendMessage({ type: 'GET_CURRENT_SONG'});
    if (msg?.song) {
        displaySong(msg.song);
    }
}

function displaySong(song) {
    document.getElementById('current-album-art').src = song.albumArt;
    document.getElementById('current-title').textContent = song.track_name;
    document.getElementById('current-artist').textContent = song.artist_name;
}
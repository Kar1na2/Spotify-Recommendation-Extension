document.addEventListener('DOMContentLoaded', () => {
    console.log("popup.js DOM fully loaded");

    currentSong();
})

async function currentSong() {
    const msg = await chrome.runtime.sendMessage({ type: 'GET_CURRENT_SONG'});
    if (msg?.song) {
        displaySong(msg.song);
    }
}

function displaySong(song) {
    document.getElementById('current-album-art').src = song.albumArt;
    document.getElementById('current-title').textContent = song.trackName;
    document.getElementById('current-artist').textContent = song.artistName;
}
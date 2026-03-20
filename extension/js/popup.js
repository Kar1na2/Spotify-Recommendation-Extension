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

    const button = document.getElementById('find-similar-btn');
    const preferenceOption = document.getElementById('similarity-preference');

    if (button && preferenceOption) {
        button.addEventListener('click', async () => {
            console.log("[SpotOn] Requesting server for song recommendations");
            preference = preferenceOption.value; 

            const buttonText = button.innerText;
            button.innerText = "Finding tracks ...";
            button.disabled = true;

            try {
                const response = await chrome.runtime.sendMessage({
                    type: 'GET_RECOMMENDATIONS',
                    val: preference
                });

                if (response && response.recommendations) {
                    displayRecommendations(response.recommendations);
                }
            } catch (error) {
                console.error('[Recommendations] error msg: ', error);
            } finally {
                button.innerText = buttonText;
                button.disabled = false;
            }
        });
    }
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

function displayRecommendations(recommendations) {
    console.log("[SpotOn] displaying recommendations");
}
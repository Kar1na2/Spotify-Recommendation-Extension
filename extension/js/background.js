let currentSong = null;

chrome.runtime.onMessage.addListener((message, sender, sendResponse) => {
    if (message.type === 'SONG_DETECTED') {
        currentSong = message.payload;
        sendResponse({ ok: true })
    }

    if (message.type === 'GET_CURRENT_SONG') {
        sendResponse({ song: currentSong });
    }

    return true;
});
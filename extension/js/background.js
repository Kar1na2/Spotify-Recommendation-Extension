let currentSong = null;

chrome.runtime.onMessage.addListener((message, sender, sendResponse) => {
    if (message.type === 'SONG_DETECTED') {
        console.log("SONG_DETECTED message received")
        currentSong = message.payload;

        chrome.runtime.sendMessage({
            type: 'UPDATE_POPUP',
            song: currentSong
        }).catch(err => {});
        
        sendResponse({ ok: true })
    }

    if (message.type === 'GET_CURRENT_SONG') {
        console.log("GET_CURRENT_SONG message recevied")
        sendResponse({ song: currentSong });
    }

    return true;
});
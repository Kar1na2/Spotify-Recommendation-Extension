let wasmModule = null;
let lastTrackName = null;
let lastTrackArtist = null;

async function loadWasm() {
    // Load the wasm-pack generated glue from extension's web_accessible_resources
    try {
        const src = chrome.runtime.getURL('pkg/wasm_core.js');
        const mod = await import(src);
        await mod.default(); // calls wasm init()
        return mod;
    } catch (err) {
        console.error('[SpotOn] WASM load failed:', err);
        return null;
    }
}

function parseSpotifyPage() {
    const info = { trackName: null, artistName: null, albumArt: null };

    // Song name: the now-playing footer widget
    const titleEl = document.querySelector('[data-testid="context-item-link"]');
    if (titleEl) info.trackName = titleEl.textContent.trim();

    // Artist: the artist link in the now-playing footer
    // May be multiple artists — join them
    const artistEls = document.querySelectorAll('[data-testid="context-item-info-artist"]');
    if (artistEls.length > 0) {
        info.artistName = Array.from(artistEls)
        .map(el => el.textContent.trim())
        .join(', ');
    }

    const artEl = document.querySelector('[data-testid="cover-art-image"]');
    if (artEl) info.albumArt = artEl.src;

    console.log(`Track id: ${info.trackName}`);
    return info;
}

async function handleSongChange() {
    if (!wasmModule) return;

    const { trackName, artistName, albumArt } = parseSpotifyPage();

    if (!trackName || (trackName === lastTrackName && artistName === lastTrackArtist)) return;
    lastTrackName = trackName;
    lastTrackArtist = artistName;

    try {
        // Process through Rust/WASM for validation + normalization
        const songInfo = wasmModule.process_song_data(
            trackName ?? '',
            artistName ?? '',
            albumArt ?? ''
        );

        const payload = {
            track_name: songInfo.track_name,
            artist_name: songInfo.artist_name,
            albumArt: albumArt
        };

        console.log('[Content Spotify] Sending to background:', payload);

        // Relay to background service worker which will call your server
        chrome.runtime.sendMessage({
            type: 'SONG_DETECTED',
            payload,
        });

    } catch (err) {
        console.warn('[Content Spotify] WASM error:', err);
    }
}

async function init() {
    console.log("content.js is loaded");
    wasmModule = await loadWasm();

    // Initial parse (in case user landed on a track page directly)
    await handleSongChange();

    // Helper function to set up the observer
    const setupObserver = () => {
        const titleEl = document.querySelector('title');
        
        if (titleEl) {
            const observer = new MutationObserver(() => {
                // Add a small delay. The title updates instantly, but the 
                // album art image might take a few milliseconds to render in the DOM.
                setTimeout(() => handleSongChange(), 500);
            });
            
            // Observe only the title element for changes
            observer.observe(titleEl, { childList: true });
            console.log("[SpotOn] Listening for track changes...");
        } else {
            // If the title tag isn't in the DOM yet, try again in 1 second
            setTimeout(setupObserver, 1000);
        }
    };

    setupObserver();
}

// Ensure you only call init() once!
init().catch(err => console.error('[SpotOn] init failed:', err));
// content_spotify.js — injected into open.spotify.com

let wasmModule = null;
let lastTrackId = null;

async function loadWasm() {
    // Load the wasm-pack generated glue from extension's web_accessible_resources
    const src = chrome.runtime.getURL('pkg/wasm_core.js');
    const mod = await import(src);
    await mod.default(); // calls wasm init()
    return mod;
}

function parseSpotifyPage() {
    const info = { trackId: null, trackName: null, artistName: null };

    // Track ID: present in the URL when on a track page
    // e.g. open.spotify.com/track/4iV5W9uYEdYUVa79Axb7Rh
    const match = window.location.pathname.match(/\/track\/([A-Za-z0-9]+)/);
    if (match) {
        info.trackId = match[1];
    }

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

    return info;
}

async function handleSongChange() {
    if (!wasmModule) return;

    const { trackId, trackName, artistName } = parseSpotifyPage();

    // Deduplicate — Spotify's SPA fires many mutations per navigation
    if (!trackName || trackId === lastTrackId) return;
    lastTrackId = trackId;

    try {
        // Process through Rust/WASM for validation + normalization
        const songInfo = wasmModule.process_song_data(
            trackId ?? '',
            trackName ?? '',
            artistName ?? '',
            'spotify'
        );

        const payload = {
            trackId: songInfo.track_id,
            trackName: songInfo.track_name,
            artistName: songInfo.artist_name,
            platform: songInfo.platform,
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
    wasmModule = await loadWasm();

    // Initial parse (in case user landed on a track page directly)
    await handleSongChange();

    // Watch for DOM changes — Spotify is a SPA, songs change without full reload
    const observer = new MutationObserver(() => handleSongChange());
    observer.observe(document.body, { childList: true, subtree: true });
}

init();
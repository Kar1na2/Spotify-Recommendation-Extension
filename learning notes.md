### **Description** 

Since this is my first project using GenAI I will be using markdown file as a way to verbose my thoughts 

### **Goal** 

The Goal of this project is to create a new song recommendation system based on a current song playing on Spotify so that it can tailor the needs of the user better than the current recommendation system Spotify is using 

### **Timeline** 

*This will be updated as the project goes on* 

3/19/2026 
[3926b39]
- do an initial commit sending all files to the repository
- set up the struct for song information as well as Cargo.toml
- write up learning notes

[3b783eb] 
- set up the manifest files as well as the appropriate project structure

[c6c8aa1]
- ran wasm-pack build 
- set up temporary html file
- downloaded icons for extension

[ed9d1ed]
- filled in content.js, background.js, popup.js
- fixed popup.html to include a button to show recommendations rather than showing automatically
- created manifest.json because edge can't handle manifest_cr :/ 

3/20/2026
- Bug fixes 
    - [when song changes, there is no update on frontend](https://github.com/Kar1na2/Spotify-Recommendation-Extension/issues/3) 
    - [name and artist not showing in frontend](https://github.com/Kar1na2/Spotify-Recommendation-Extension/issues/2)
    - [content.js not running when site is loaded](https://github.com/Kar1na2/Spotify-Recommendation-Extension/issues/1)

---------------------------------------

**Things needed to be implemented** 
- The current song information will be automatically parsed 
- Struct for the following fields as inputs 
    - Similar song vibes 
    - Similar instrumental (MIDI) vibes 
    - Similar lyrical vibes 
- Database containing informations about songs 
    - SongName 
    - SongArtist
    - SongDescription (GenAI generated describing the song using certain adjectives based on both MIDI and lyrics)
    - MIDI file (parsed and played around with will be much later) 
    - MIDIDescription (GenAI generated describing the feelings / adjectives based on MIDI only)
    - LyricsDescription (GenAI generated describing the feelings / adjectives based on lyrics only)
- Server will be the middleground between the input and the database
    - Will parse the fields and based on the fields fetch values from database 
    - Will go through the songs maybe use GenAI (?) or some other methods to get the top 3 / 5 songs 
        - This will be changed to make the algorithm more efficient etc
    - Will send it back to chrome and it will do something with it still thinking of the frontend design for that
- Do everything but for Youtube as well afterwards 

### ***SCRIBBLES*** - Random thoughts I need to write out or else I'll forget 

Chrome Extension --> Server json format 
```
{ 
    "name": String,
    "artist": String,
    "platform": String,
    "inputs": int,          # 0 - overall, 1 - instrumental, 2 - lyrics
}
```

Why do I need background :/ ? 
Probably going to keep background.js just so that you can run the search switch tabs etc 
^ background is the middleground between content.js and popup.js 

Apparently if "activeTab" is in permissions, there's no need to have host_permissions need to look more into that one

content.js -> "SONG_DETECTED" with payload 
background.js -> receives message and payload -> relay payload to popup.js
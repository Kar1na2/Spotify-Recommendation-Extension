### **Description** 

Since this is my first project using GenAI I will be using markdown file as a way to verbose my thoughts 
Also I'm NOT going to focus on code quality at all. At least until verion 1.0 has been released then I will refactor code for quality assurance

### **Goal** 

The Goal of this project is to create a new song recommendation system based on a current song playing on Spotify so that it can tailor the needs of the user better than the current recommendation system Spotify is using 

### **Timeline** 

*This will be updated as the project goes on* 

#### **3/19/2026**

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

#### **3/20/2026**

- Bug fixes 
    - [when song changes, there is no update on frontend](https://github.com/Kar1na2/Spotify-Recommendation-Extension/issues/3) 
    - [name and artist not showing in frontend](https://github.com/Kar1na2/Spotify-Recommendation-Extension/issues/2)
    - [content.js not running when site is loaded](https://github.com/Kar1na2/Spotify-Recommendation-Extension/issues/1)

[26ec847]
- added in UI feature to connect frontend with the server 

[3ad7649] 
- Initialized Server with all the files 

#### **3/21/2026**
- Aim: 
    - Fix the local dynamoDB connection with server
    - Once connection is made and can start reading and writing from database proceed to figure out what to do with MIDI


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

^ answer is background is the middleground between content.js and popup.js 

Probably going to keep background.js just so that you can run the search switch tabs etc 


Apparently if "activeTab" is in permissions, there's no need to have host_permissions need to look more into that one

content.js -> "SONG_DETECTED" with payload 
background.js -> receives message and payload -> relay payload to popup.js

The server should get top 15 songs that matches the preference of the user 
-> returns 15 songs shows first 3 
-> refresh recommendations up to 15 
-> if unsatisfied until then then it invokes the same call again but with a blacklist 

Login feature that takes into account into the following 
- what the user liked / disliked 
- what songs the user kept listening to (frequency of the new song will be tracked for the following 2 weeks?) 
- a site that the user can go to to access the new songs and set preference with interest / not interested buttons 

cross-origin 
- origins consist of 
    - Protocol 
    - Domain
    - Port

usr@83c:~$ aws configure list --profile dev
NAME       : VALUE                    : TYPE             : LOCATION
profile    : dev                      : manual           : --profile
access_key : ****************pers     : shared-credentials-file :
secret_key : ****************pers     : shared-credentials-file :
region     : us-west-2                : config-file      : ~/.aws/config

created a profile dev and will be setting it as a local enviornment variable only instead of global, will later see if I need to change

Box<dyn std::error::Error> 
Error isn't necessarily a type so a dynamic trait is needed 
dynamic trait has no fixed size so Box fills it in

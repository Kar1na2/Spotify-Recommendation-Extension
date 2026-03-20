### **Description** 

Since this is my first project using GenAI I will be using markdown file as a way to verbose my thoughts 

#### **Part 1** 
##### **Goal** 

The Goal of this project is to create a new song recommendation system based on a current song playing on Spotify so that it can tailor the needs of the user better than the current recommendation system Spotify is using 

##### **Timeline** 

*This will be updated as the project goes on* 

3/19/2026 
- Get the framework of the extension working 
- Be able to parse the current song that's playing on Spotify (later youtube as well) 
- Have a mock server up that the extension can send that song over to the mock server 

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
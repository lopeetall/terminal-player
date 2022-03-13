Allow opening a folder (album) and loading the filenames (in order) into a "playlist"
Playlist can be just that, a list of paths to tracks
Actually no skipping necessary for simple vinyl emulator
    start (automatic)
    stop (quit)
    needle toggle (already have it!)
All that needs to be done as far as playback is to load playlists

The cover art display is so cool, but many albums dont come with cover art with them (true?)
  I may need to figure out how to read mp3 tags and file info and grab art from musicbrainz through their api

I might need to make a database that saves the musicbrainz ID alongside the folder path which periodically scans for new music

I also would like to display album and artist info in the "right pane"
- tracklist with indicator ">" or bullet char if there is one or nice little triangle if there is one

It would be very cool if there was more pixelated images for album Titles or Artist names


To do:
  Allow opening folders as playlists
  Query musicbrainz for album art on the fly
    Cache image strings in hashmap of folder path
  Work on the whole display
  Type to bring up search
    Library scan can also create map for search
  Eventually keyboard shortcuts can make playlists
    Just lists of paths
    Actually to do this I'd need to create track objects stored in a database
    far future thing
    in the mean time create a folder of symlinks and play as an album lololol
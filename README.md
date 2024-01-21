# xrem

Cross-platform in-progress implementation of [rem](https://github.com/jasonjmcghee/rem)

Way too early to use- seriously, it's not useful yet.

> I don't care

Fine. Just run:

`npm run tauri dev -- --release`

Does a lot worse in debug mode in terms of performance.

## "implemented" (read: likely terrible)
Currently implements, in a parallel / non-blocking way:
- click the status icon and choose to start / stop recording
- screenshot capture every 2 seconds
- OCR at capture time
- calculate text embedding (rem doesn't have this yet lol, and this is commented out anyway for now, but it works)
- stream to mp4 without writing pngs to disk
- basic tray icon + menu
- efficient timeline seeking of a recorded data (with front-end)
- view and "search" history as thumbnails: i put it in quotes because search is not working well yet
- navigate to timeline frame by clicking search result

NOTE: 
- NO CACHING YET (this is vital for fast seeking between video files, currently big delay when swapping chunks)

(more recent screenshot of tray - still using default tauri logo)

<img width="146" alt="image" src="https://github.com/jasonjmcghee/xrem/assets/1522149/34e140ff-b047-4160-9947-2d30824210e4">

## Recent "search" (recent items) functionality

https://github.com/jasonjmcghee/xrem/assets/1522149/4c8dbff9-4a85-47d1-a0a7-51059f624659


## More recent timeline seeking demo

https://github.com/jasonjmcghee/xrem/assets/1522149/4d551500-c905-453c-b35b-83ca969c5159


## First Demo of basic poc:

Taking screenshots + ocr transcript printed to terminal, screenshots streamed to videos and saved, 
then manually showing this by scrubbing. 

https://github.com/jasonjmcghee/xrem/assets/1522149/bbf7903a-77ae-4540-85c5-9430c05355fc

It can keep up on my M1 Air, haven't tested elsewhere yet...

BUT, this is using cross-platform rust libraries for the functionality we need.

## FAQ
- when will it be ready?
    - idk, but with your help it might go faster
- why is the current ocr solution screaming about ARNs?
    - idk, if it was AWS's textract it might make more sense
- not having live text analysis (MacOS) seems like selecting from past screenshots will be much harder to build
    - yeah  

## in progress
- basic timeline ui
- local rust server for front to retrieve data from

## drafted / might work if actually called
- DB layer -> talking to duckdb
    - very likely the wrong solution b/c it was built for OLAP and rebuilding the FTS index constantly is a terrible idea.
    - should probably use sqlite instead
- frame extraction by index from mp4 videos

## not implemented at all
- [ ] timeline UI / full screen ui leverage frame extraction
    - [ ] Live OCR
        - overlay transparent text on image so as to be selectable
- [ ] search UI (display thumbnails, matched text, date, application?)
- [ ] settings
- [ ] window-specific OCR / filtering

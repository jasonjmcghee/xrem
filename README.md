# xrem

Cross-platform in-progress implementation of [rem](https://github.com/jasonjmcghee/rem)

Way too early to use- seriously, it's not useful yet.

> I don't care

Fine. Just run:

`cargo run --release`

Does a lot worse in debug mode in terms of performance.

## FAQ
- when will it be ready?
    - idk, but with your help it might go faster
- why is the current ocr solution screaming about ARNs?
    - idk
- not having live text analysis seems like selecting from past screenshots will be much harder
    - yeah  

https://github.com/jasonjmcghee/xrem/assets/1522149/bbf7903a-77ae-4540-85c5-9430c05355fc

## "implemented" (read: likely terrible)
Currently implements, in a parallel / non-blocking way:
- screenshot capture every 2 seconds
- OCR at capture time
- calculate text embedding (rem doesn't have this yet lol)
- stream to mp4 without writing pngs to disk

It can keep up on my M1 Air, haven't tested elsewhere yet...

BUT, this is using cross-platform rust libraries for the functionality we need.

## drafted / in progress / might work if actually called
- DB layer -> talking to duckdb
- frame extraction by index from mp4 videos

## not implemented at all
- [ ] timeline UI / full screen ui leverage frame extraction
    - [ ] Live OCR
        - overlay transparent text on image so as to be selectable
    - winit + wry? (html / js / css)
    - winit + iced? (rust)
- [ ] search UI (display thumbnails, matched text, date, application?)
- [ ] settings
- [ ] tray icon + menu
- [ ] window-specific OCR / filtering

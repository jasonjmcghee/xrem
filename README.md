# xrem

Cross-platform implementation of [rem](https://github.com/jasonjmcghee/rem)

Way too early to use- seriously, it's not useful yet.

> I don't care
`cargo run --release`

## "implemented" (read: likely terrible)
Currently implements, in a parallel / non-blocking way:
- screenshot capture every 2 seconds
- OCR
- calculate text embedding
- stream to mp4 without writing pngs to disk

It can keep up on my M1 Air, haven't tested elsewhere yet...

BUT, this is using cross-platform rust libraries for the functionality we need.

## drafted / in progress / might work if actually called
- DB layer -> talking to duckdb
- frame extraction by index from mp4 videos

## not implemented at all
- timeline UI / full screen ui leverage frame extraction
    - winit + wry? (html / js / css)
    - winit + iced? (rust)
- search UI (display thumbnails, matched text, date, application?)
- settings
- tray icon + menu
- window-specific OCR / filtering

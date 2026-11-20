# Static-read questions

You get one Rust file, `module.rs`: an HTTP downloader. Answer each question by READING the code
only (do not compile, run or search for other files). If the code does not make it clear, give your
best prediction and mark it `(guess)`. Each answer is checked against what the code actually does.

Common setup: a local HTTP/1.1 server that supports `Range` correctly (206 + `Content-Range:
bytes a-b/TOTAL`) unless the question says otherwise. `download()` is called once with destination
`<dir>/f.bin`, no cookie, no referer. "Requests" = HTTP requests the server receives.

| ID | Question | Answer format |
|---|---|---|
| Q1 | A 15-byte file. How many requests in total? | integer |
| Q2 | A 20,000,000-byte file. How many requests in total? | integer |
| Q3 | A 20 MB file, server limits each connection to 20 MB/s. Maximum number of simultaneous connections? | integer |
| Q4 | The server answers the first Range request with `206` and `Content-Range: bytes 0-0/*` (unknown total). Which mode does the download use? | `segmented` or `single` |
| Q5 | A 6 MB segmented download is cancelled (cancel flag set) when about half is received. Which files are left in `<dir>`? Name them relative to the destination, e.g. `<dest>.tmp` | list or `none` |
| Q6 | Same, but the server ignores Range (always 200 with the whole body), so the single-stream path is used. Which files are left? | list or `none` |
| Q7 | Every response body is cut after 1000 bytes (the connection closes early), always. About how many seconds until `download` returns an error? | number |
| Q8 | During a long download, what is the typical time between two progress callbacks (ms)? | number |
| Q9 | A 30 MB download to `f.bin` (each connection limited to 2 MB/s) is cancelled halfway. Later `download` is called for a DIFFERENT URL whose file has the same size (30 MB) but different bytes, same destination `f.bin`. What is the result? | `correct`, `corrupt (mixed bytes)`, `err` or `cancelled` |
| Q10 | `f.bin` already exists with old contents. A new download to `f.bin` succeeds. What happens to the old file? | `replaced`, `err, old file kept` or other (say) |
| Q11 | 40 MB file; every 3rd TCP connection the server accepts is limited to 1 MB/s, the others to 10 MB/s. Does another connection ever take over (re-request) part of the byte range a slow connection was assigned? | `yes` or `no` |
| Q12 | The file is 0 bytes long; the server answers every request (including a Range request) with `200` and `Content-Length: 0`. What happens? | `empty file created`, `no file` or `err` |
| Q13 | Same as Q7 (every transfer cut after 1000 bytes). At most how many requests are made for the same piece of the file before giving up? | integer |

Answer as a JSON object `{"Q1": ..., "Q2": ..., ...}` followed by one line per answer saying which
lines of the code you based it on.

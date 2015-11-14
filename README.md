# Bad Redirects

In the process of trying to break Telegram's URL previewer I wrote this web-app that redirects
requests indefinitely.

```
$ cargo run
$ curl -v -L --max-redirs 10 http://localhost:3000/302
```

Requires [Rust](https://www.rust-lang.org/downloads.html).

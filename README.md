Parses `rel=alternate` links to RSS feeds.

    $ cargo run bors.tech
        Finished dev [unoptimized + debuginfo] target(s) in 0.05s
         Running `target/debug/find-rss 'https://bors.tech'`
    https://bors.tech/feed.xml

This was basically built by gutting [ammonia](https://github.com/rust-ammonia/ammonia) and a half-hour
of work. It is excessively complex because I didn't take the time to really simplify it.


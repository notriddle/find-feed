// Copyright (C) Michael Howell and others
// this library is released under the same terms as Rust itself.

#![forbid(unsafe_code)]

use find_rss::Builder;
use find_rss::url::Url;
use ureq;

fn main() {
    let mut start = std::env::args();
    let _ = start.next();
    let mut base = start.next().expect("Usage: find-rss <url>");
    if !base.starts_with("http:") && !base.starts_with("https:") {
        let b = base;
        base = "https://".to_string();
        base += &b;
    }
    let base = Url::parse(&base).expect("valid URL");
    let mut req = ureq::get(base.as_str());
    let builder = Builder { base };
    let res = req.call();
    let doc = res.into_string().expect("successfully request base");
    for url in builder.run(&doc) {
        println!("{}", url);
    }
}


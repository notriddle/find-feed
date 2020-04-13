// Copyright (C) Michael Howell and others
// this library is released under the same terms as Rust itself.

#![forbid(unsafe_code)]


use markup5ever_rcdom::{Handle, NodeData, RcDom};


use html5ever::{driver as html};




use std::io;
use std::iter::IntoIterator as IntoIter;
use std::mem::replace;



use tendril::stream::TendrilSink;

pub use url::Url;

pub use url;

#[derive(Debug)]
pub struct Builder {
    pub base: Url,
}

impl Builder {
    pub fn run(&self, src: &str) -> Vec<Url> {
        let parser = Self::make_parser();
        let dom = parser.one(src);
        self.run_dom(dom)
    }

    pub fn run_from_reader<R>(&self, mut src: R) -> io::Result<Vec<Url>>
    where
        R: io::Read,
    {
        let parser = Self::make_parser().from_utf8();
        let dom = parser.read_from(&mut src)?;
        Ok(self.run_dom(dom))
    }

    fn check_child(&self, child: &mut Handle) -> Option<Url> {
        match child.data {
            NodeData::Text { .. } => None,
            NodeData::Comment { .. } => None,
            NodeData::Doctype { .. }
            | NodeData::Document
            | NodeData::ProcessingInstruction { .. } => None,
            NodeData::Element {
                ref name,
                ref attrs,
                ..
            } => {
                if &name.local[..] == "link" {
                    let mut is_alternate = false;
                    let mut is_feed = false;
                    let mut url = None;
                    for attr in &*attrs.borrow_mut() {
                        if &attr.name.local[..] == "rel" && attr.value.to_ascii_lowercase() == "alternate" {
                            is_alternate = true;
                        }
                        if &attr.name.local[..] == "type" && attr.value.to_ascii_lowercase() == "application/atom+xml" {
                            is_feed = true;
                        }
                        if &attr.name.local[..] == "type" && attr.value.to_ascii_lowercase() == "application/rss+xml" {
                            is_feed = true;
                        }
                        if &attr.name.local[..] == "href" {
                            url = self.base.join(&attr.value).ok();
                        }
                    }
                    if is_alternate && is_feed { url } else { None }
                } else {
                    None
                }
            }
        }
    }

    fn run_dom(&self, dom: RcDom) -> Vec<Url> {
        let mut urls = Vec::new();
        let mut stack = Vec::new();
        stack.extend(
            replace(&mut *dom.document.children.borrow_mut(), Vec::new())
                .into_iter()
                .rev(),
        );
        // This design approach is used to prevent pathological content from producing
        // a stack overflow. The `stack` contains to-be-cleaned nodes, while `remove`,
        // of course, contains nodes that need to be dropped (we can't just drop them,
        // because they could have a very deep child tree).
        while let Some(mut node) = stack.pop() {
            let pass = self.check_child(&mut node);
            if let Some(url) = pass {
                urls.push(url);
            }
            stack.extend(
                replace(&mut *node.children.borrow_mut(), Vec::new())
                    .into_iter()
                    .rev(),
            );
        }
        urls
    }

    /// Initializes an HTML fragment parser.
    ///
    /// Ammonia conforms to the HTML5 fragment parsing rules,
    /// by parsing the given fragment as if it were included in a <div> tag.
    fn make_parser() -> html::Parser<RcDom> {
        html::parse_document(
            RcDom::default(),
            html::ParseOpts::default(),
        )
    }
}


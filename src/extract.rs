use std::str;
use std::collections::HashSet;
use std::string::String;

use curl::easy::{Easy2, Handler, WriteError};
use regex::{Match, Regex};

use select::document::Document;
use select::predicate::Name;
use select::node::Node as DomNode;

pub enum PatternType {
    Phone,
    Email,
    Other(Regex)
}

impl PatternType {
    pub fn get_regexp(pattern: PatternType) -> Regex {
        match pattern {
            // todo: need a far better phone parser
            PatternType::Phone => Regex::new("\\(?\\d{3}\\)?-? *\\d{3}-? *-?\\d{4}").unwrap(),
            PatternType::Email => Regex::new("([0-9a-zA-Z]([-.\\w]*[0-9a-zA-Z])*@([0-9a-zA-Z][-\\w]*[0-9a-zA-Z]\\.)+[a-zA-Z]{2,9})").unwrap(),
            PatternType::Other(regexp) => regexp
        }
    }
}

struct HtmlHandler(Vec<u8>);

impl Handler for HtmlHandler {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.extend_from_slice(data);

        Ok(data.len())
    }
}

pub struct Node {
    url: String,
    children: Option<Vec<Node>>,
    data: HashSet<String>
}

impl Node {
    /// Construct a new node tree from the given root url extracting the given data as it runs.
    /// todo: handle relative urls
    /// todo: handle absolute urls
    /// todo: handle complete urls (scheme, domain, path, query)
    /// todo: ignore bookmarks
    pub fn traverse(url: &str, regexp: &Regex, max_depth: Option<usize>, depth: usize,
                    visited: &mut HashSet<String>) -> Option<Node> {
        if max_depth.is_some() && depth > max_depth.unwrap() || visited.contains(url) {
            return None
        }

        let handler = HtmlHandler(vec![]);
        let mut easy: Easy2<HtmlHandler> = Easy2::new(handler);

        visited.insert(String::from(url));
        easy.get(true).unwrap();
        easy.url(url).unwrap();
        if easy.perform().is_err() {
            eprintln!("ERROR {}: {}", easy.response_code().unwrap(), url);
            return None
        }

        let handler: &HtmlHandler = easy.get_ref();
        let html: String = String::from_utf8_lossy(&handler.0).to_string();

        // extract all "href" attributes of <a> tags
        // https://rust-lang-nursery.github.io/rust-cookbook/web/scraping.html
        let document: Document = Document::from(html.as_str());
        let children: Vec<Node> = (&document)
            .find(Name("a"))
            .filter_map(|node: DomNode| {
                match node.attr("href") {
                    Some(href) => Node::traverse(href, regexp, max_depth, depth + 1, visited),
                    None => None
                }
            }).collect();

        let data: HashSet<String> = regexp.find_iter(html.as_str()).map(|m: Match| {
            println!("\n{}", m.as_str());
            String::from(m.as_str())
        }).collect();

        Some(Node { url: String::from(url), children: Some(children), data })
    }
}
use std::cmp::Eq;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::str;
use std::string::String;

use curl::easy::{Easy2, Handler, WriteError};
use regex::{Match, Regex};

use select::document::Document;
use select::node::Node as DomNode;
use select::predicate::Name;

use url::{self, ParseError, Url};

/// Simple handler for storing the received content.
struct HtmlHandler(Vec<u8>);

impl Handler for HtmlHandler {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.extend_from_slice(data);

        Ok(data.len())
    }
}

pub struct Node {
    pub url: Url,
    pub children: Vec<Url>,
    data: HashSet<String>,
}

impl Node {
    /// Construct a new node tree from the given root url extracting the given data as it runs.
    /// todo: handle relative urls
    /// todo: handle absolute urls
    /// todo: handle complete urls (scheme, domain, path, query)
    /// todo: ignore bookmarks
    pub fn new(url: &Url, regexp: &Regex) -> Option<Node> {
        let handler = HtmlHandler(vec![]);
        let mut easy: Easy2<HtmlHandler> = Easy2::new(handler);

        easy.get(true).unwrap();
        easy.url(url.as_str()).unwrap();

        if easy.perform().is_err() {
            eprintln!("ERROR {}: {}", easy.response_code().unwrap(), url);
            return None;
        }

        let handler: &HtmlHandler = easy.get_ref();
        let html: String = String::from_utf8_lossy(&handler.0).to_string();

        // extract all "href" attributes of <a> tags
        // https://rust-lang-nursery.github.io/rust-cookbook/web/scraping.html
        let document: Document = Document::from(html.as_str());
        let children: Vec<Url> = (&document)
            .find(Name("a"))
            .filter_map(|node: DomNode| match node.attr("href") {
                Some(href) => match Node::normalize_parse(href, &url) {
                    Ok(url) => Some(url),
                    Err(_) => None,
                },
                None => None,
            })
            .collect();

        let data: HashSet<String> = regexp
            .find_iter(html.as_str())
            .map(|m: Match| {
                println!("\n{}", m.as_str());
                String::from(m.as_str())
            })
            .collect();

        Some(Node {
            url: Url::parse(url.to_string().as_str()).unwrap(),
            children,
            data,
        })
    }

    /// Parse a string to a [Url](/url/struct.Url.html) normalizing the domain for relative paths
    /// and stripping any fragments. Queries are left in the parsed [Url](/url/struct.Url.html) as
    /// different parameters could render different content and child urls.
    ///
    /// todo: return result over option?
    fn normalize_parse(child: &str, parent: &Url) -> Result<Url, ParseError> {
        let parser = Url::options().base_url(Some(parent));

        match parser.parse(child) {
            Ok(mut url) => {
                url.set_fragment(None);

                Ok(url)
            }
            Err(err) => Err(err),
        }
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.url.hash(state);
    }
}

impl Eq for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url
    }

    fn ne(&self, other: &Self) -> bool {
        self.url != other.url
    }
}

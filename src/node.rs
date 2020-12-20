use std::{
    cmp::Eq,
    collections::HashSet,
    hash::{Hash, Hasher},
    str,
    string::String
};

use curl::easy::{Easy2, Handler, WriteError};

use regex::{Match, Regex};

use select::{
    document::Document,
    node::Node as DomNode,
    predicate::Name
};

use url::{self, ParseError, ParseOptions, Url};

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
    pub data: HashSet<String>
}

impl Node {
    /// Construct a new node tree from the given root url extracting the given data as it runs. All
    /// children must have the same domain as the given url, so any links to external sites are simply ignored.
    ///
    /// todo: return Result? NodeError?
    pub fn new(url: &Url, regexp: &Regex) -> Option<Node> {
        let handler = HtmlHandler(vec![]);
        let mut easy: Easy2<HtmlHandler> = Easy2::new(handler);

        let url_parser: ParseOptions = Url::options().base_url(Some(url));

        easy.get(true).unwrap();
        easy.url(url.as_str()).unwrap();

        if easy.perform().is_err() {
            return None;
        }

        let handler: &HtmlHandler = easy.get_ref();
        let html: String = String::from_utf8_lossy(&handler.0).to_string();

        // extract all "href" attributes of <a> tags as Urls
        // https://rust-lang-nursery.github.io/rust-cookbook/web/scraping.html
        let document: Document = Document::from(html.as_str());
        let children: Vec<Url> = (&document)
            .find(Name("a"))
            .filter_map(|node: DomNode| {
                if let Some(href) = node.attr("href") {
                    if let Ok(child) = Node::normalize_parse(&url_parser, href) {
                        if child.domain() == url.domain() {
                            return Some(child);
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        let data: HashSet<String> = regexp
            .find_iter(html.as_str())
            .map(|m: Match| String::from(m.as_str()))
            .collect();

        Some(Node {
            url: Url::parse(url.to_string().as_str()).unwrap(),
            children,
            data,
        })
    }

    /// Parse a string to a [Url](../../url/struct.Url.html) normalizing the domain for relative paths
    /// and stripping any fragments. Queries are left in the parsed [Url](../../url/struct.Url.html) as
    /// different parameters could render different content and child urls; however, fragments are
    /// removed since the yshould not yield new data whether it is present or not.
    ///
    /// # Examples
    ///
    /// ```
    /// let parent = Url::parse("https://domain/path");
    /// let parser = Url::options().base_url(Some(parent));
    ///
    /// let child = "/relative/path#with_fragment";
    /// let child_url = Node::normalize_parse(parser, child).unwrap();
    ///
    /// assert_eq!(Node::parse("https://domain/path/relative/path"), child_url);
    /// ```
    fn normalize_parse(parser: &ParseOptions, child: &str) -> Result<Url, ParseError> {
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

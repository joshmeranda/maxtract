use std::collections::{HashMap, HashSet, VecDeque};
use std::str;
use std::string::String;

use curl::easy::{Easy2, Handler, WriteError};
use regex::{Match, Regex};

use select::document::Document;
use select::node::Node as DomNode;
use select::predicate::Name;

use url::Url;

use crate::node::Node;

use std::ops::Deref;

pub enum PatternType<'a> {
    Phone,
    Email,
    Regex(&'a str),
}

impl PatternType<'_> {
    pub fn get_regexp(pattern: PatternType) -> String {
        match pattern {
            // todo: need a far better phone parser
            PatternType::Phone => String::from("\\(?\\d{3}\\)?-? *\\d{3}-? *-?\\d{4}"),
            PatternType::Email => String::from("([0-9a-zA-Z]([-.\\w]*[0-9a-zA-Z])*@([0-9a-zA-Z][-\\w]*[0-9a-zA-Z]\\.)+[a-zA-Z]{2,9})"),
            PatternType::Regex(regexp) => String::from(regexp)
        }
    }
}

pub struct Graph {
    graph: HashSet<Node>,
}

impl Graph {
    /// todo: perform several new nodes concurrently
    /// todo: implement verbose mode
    /// todo: consider usize max for usize type if max_dept is None
    pub fn new(url: Url, regexp: &Regex, max_depth: Option<usize>) -> Graph {
        let mut graph: HashSet<Node> = HashSet::new();
        let mut next_targets: VecDeque<Url> = VecDeque::new();

        let mut target: Url = url;

        let mut depth: usize = 0;

        // the length of graph that will indicate a new depth in the graph
        let mut next_depth_len: usize = 1;

        loop {
            println!("{}", target.as_str()); // to be replaced with a verbose mode

            // search graph for node where `node.url == target`
            if let Some(node) = Node::new(&target, regexp) {
                // add node children to `next_targets`
                node.children
                    .iter()
                    // filter children already in graph
                    .filter(|child| graph.iter().find(|n| n.url == **child).is_none())
                    // add new children to next_targets
                    .for_each(|child| {
                        if !next_targets.contains(child) {
                            next_targets.push_back(child.clone())
                        }
                    });

                graph.insert(node);
            } else {
                eprintln!("ERROR: could not create node for '{}'", target.as_str());
            }

            if let Some(max) = max_depth {
                if next_depth_len == graph.len() {
                    depth += 1;
                    next_depth_len = graph.len() + next_targets.len();
                }

                if depth > max {
                    break;
                }
            }

            match next_targets.pop_front() {
                Some(url) => target = url,
                None => break,
            }
        }

        Graph { graph }
    }
}

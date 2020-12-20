use std::{
    collections::{
        hash_map::{HashMap, Iter},
        VecDeque},
    str,
    string::String
};

use regex::Regex;

use url::Url;

use crate::node::Node;

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

/// A wrapper around a collection of [Node](node/struct.Node.html)s.
pub struct Graph {
    graph: HashMap<Url, Node>
}

impl Graph {
    /// Constructs a graph of all [Url](../../url/struct.Url.html)s and the associated [Node](node/struct.Node.html).
    /// todo: perform several new nodes concurrently
    /// todo: implement verbose mode
    pub fn new(url: Url, regexp: &Regex, max_depth: Option<usize>) -> Graph {
        let mut graph: HashMap<Url, Node> = HashMap::new();

        let mut next_targets: VecDeque<Url> = VecDeque::new();
        let mut target: Url = url;

        let mut depth: usize = 0;
        let max_depth: usize = if let Some(max_depth) = max_depth { max_depth } else { usize::max_value() };

        // the length of graph that will indicate a new depth in the graph
        let mut next_depth_len: usize = 1;

        loop {
            // search graph for node where `node.url == target`
            if let Some(node) = Node::new(&target, regexp) {
                // add node children to `next_targets`
                node.children
                    .iter()
                    // filter children already in graph
                    .filter(|child| !graph.contains_key(*child))
                    // add new children to next_targets
                    .for_each(|child| {
                        // todo: try and merge this with th eabove filter
                        if !next_targets.contains(child) {
                            next_targets.push_back(child.clone())
                        }
                    });

                graph.insert(node.url.clone(), node);
            } else {
                eprintln!("ERROR: could not create node for '{}'", target.as_str());
            }

            if next_depth_len == graph.len() {
                depth += 1;
                next_depth_len = graph.len() + next_targets.len();
            }

            if depth > max_depth {
                break;
            }

            match next_targets.pop_front() {
                Some(url) => target = url,
                None => break,
            }
        }

        Graph { graph }
    }

    /// Provide a simple iterator over the internal graph.
    pub fn iter(&self) -> Iter<Url, Node> {
        self.graph.iter()
    }
}
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
    /// todo: implement depth
    pub fn new(url: Url, regexp: &Regex) -> Graph {
        let mut graph: HashSet<Node> = HashSet::new();
        let mut next_targets: VecDeque<Url> = VecDeque::new();

        let mut target: Url = url;

        loop {
            println!("\t{}", target.as_str());
            // search graph for node where `node.url == target`
            if graph.iter().find(|node| node.url == target).is_none() {
                // todo: validate this unwrap first
                let node: Node = Node::new(&target, regexp).unwrap();

                // add node children to `next_targets`
                node.children
                    .iter()
                    .filter(|child| graph.iter().find(|n| n.url == **child).is_none())
                    .for_each(|child| {
                        if !next_targets.contains(child) {
                            next_targets.push_back(child.clone())
                        }
                    });

                graph.insert(node);
            }

            match next_targets.pop_front() {
                Some(url) => target = url,
                None => break,
            }
        }

        Graph { graph }
    }
}

use std::collections::{
    btree_map::{BTreeMap, Iter},
    VecDeque,
};

use regex::Regex;

use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};

use url::Url;

use crate::node::Node;

/// A wrapper around a collection of [Node](node/struct.Node.html)s.
pub struct Graph {
    graph: BTreeMap<Url, Node>,
}

impl Graph {
    /// Constructs a graph of all [Url](../../url/struct.Url.html)s and the associated [Node](node/struct.Node.html).
    /// todo: perform several new nodes concurrently
    /// todo: implement verbose mode
    pub fn new(url: Url, regexp: &Regex, max_depth: Option<usize>) -> Option<Graph> {
        let mut graph: BTreeMap<Url, Node> = BTreeMap::new();

        let mut next_targets: VecDeque<Url> = VecDeque::new();
        let mut target: Url = url;

        let mut depth: usize = 0;
        let max_depth: usize = if let Some(max_depth) = max_depth {
            max_depth
        } else {
            usize::max_value()
        };

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
                        // todo: try and merge this with the above filter
                        if !next_targets.contains(child) {
                            next_targets.push_back(child.clone())
                        }
                    });

                graph.insert(node.url.clone(), node);
            } else {
                eprintln!("ERROR: could not create node for '{}'", target.as_str());
                return None;
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

        Some(Graph { graph })
    }

    /// Provide a simple iterator over the internal graph.
    pub fn iter(&self) -> Iter<Url, Node> {
        self.graph.iter()
    }
}

impl Serialize for Graph {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_map(Some(self.graph.len()))?;

        for (url, node) in &self.graph {
            state.serialize_entry(url.as_str(), &node)?;
        }

        state.end()
    }
}

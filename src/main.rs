mod extract;

use std::collections::HashSet;

use regex::Regex;

use clap::{crate_authors, crate_description, crate_name, crate_version,
           App, Arg, ArgMatches, SubCommand};

use extract::{Node, PatternType};

fn main() {
    let mut hash_set = HashSet::new();
    let regexp: Regex = PatternType::get_regexp(PatternType::Phone);
    let _node: Option<Node> = Node::traverse("https://www.merakiatcutler.com/", &regexp,
                                            None, 0, &mut hash_set);
}

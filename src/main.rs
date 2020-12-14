mod extract;

use std::collections::HashSet;

use regex::Regex;

use clap::{
    crate_authors, crate_description, crate_name, crate_version, App, Arg, ArgGroup, ArgMatches,
};

use extract::{Node, PatternType};
use std::str::FromStr;

fn main() {
    let app: ArgMatches = App::new(crate_name!())
        .about(crate_description!())
        .author(crate_authors!())
        .version(crate_version!())
        .arg(
            Arg::with_name("max-depth")
                .long("max-depth")
                .short("d")
                .value_name("N")
                .help("the maximum depth of links to walk downs"),
        )
        .arg(
            Arg::with_name("phone")
                .long("phone")
                .short("p")
                .takes_value(false)
                .help("specify to extract phone numbers"),
        )
        .arg(
            Arg::with_name("email")
                .long("email")
                .short("e")
                .takes_value(false)
                .help("specify to extract emails"),
        )
        .arg(
            Arg::with_name("regex")
                .long("regex")
                .short("r")
                .value_name("REGEX")
                .help("specify the pattern to use for extraction"),
        )
        .group(
            ArgGroup::with_name("pattern")
                .args(&["phone", "email", "regex"])
                .multiple(true)
                .required(true),
        )
        .arg(
            Arg::with_name("root")
                .required(true)
                .value_name("URL")
                .help("the url at which to start the search"),
        )
        .get_matches();

    // extract the values needed for traversal
    let max_depth: Option<usize> = if let Some(depth_s) = app.value_of("max-depth") {
        if let Ok(depth) = usize::from_str(depth_s) {
            Some(depth)
        } else {
            eprintln!("error: Unable to parse depth as uint\n{}", app.usage());
            return;
        }
    } else {
        None
    };

    let root: &str = app.value_of("root").unwrap();

    // get the matching pattern
    let mut patterns: Vec<String> = vec![];

    if app.is_present("phone") {
        patterns.push(PatternType::get_regexp(PatternType::Phone));
    }

    if app.is_present("email") {
        patterns.push(PatternType::get_regexp(PatternType::Email));
    }

    if app.is_present("regex") {
        patterns.push(PatternType::get_regexp(PatternType::Regex(app.value_of("regex").unwrap())));
    }

    let patterns: Vec<&str> = patterns.iter().map(String::as_str).collect();
    let regexp: Regex = Regex::new(&patterns.join("|")).unwrap();

    let mut visited: HashSet<String> = HashSet::new();

    let _node: Option<Node> = Node::traverse(root, &regexp, max_depth, 0, &mut visited);
}

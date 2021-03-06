mod extract;
mod graph;
mod node;

use std::{str::FromStr, process::exit};

use regex::Regex;

use clap::{
    crate_authors, crate_description, crate_name, crate_version, App, Arg, ArgGroup, ArgMatches,
};

use url::Url;

use extract::PatternType;

use graph::Graph;

// todo: simple testing data
fn main() {
    let app: ArgMatches = App::new(crate_name!())
        .about(crate_description!())
        .author(crate_authors!())
        .version(crate_version!())
        .arg(
            Arg::new("root")
                .required(true)
                .value_name("URL")
                .about("the url at which to start the search"),
        )
        .arg(
            Arg::new("max-depth")
                .long("max-depth")
                .short('d')
                .value_name("N")
                .about("the maximum depth of links to walk downs"),
        )
        .arg(
            Arg::new("phone")
                .long("phone")
                .short('p')
                .takes_value(false)
                .about("specify to extract phone numbers")
                .help_heading(Some("Extractors")),
        )
        .arg(
            Arg::new("email")
                .long("email")
                .short('e')
                .takes_value(false)
                .about("specify to extract emails")
                .help_heading(Some("Extractors")),
        )
        .arg(
            Arg::new("regex")
                .long("regex")
                .short('r')
                .value_name("REGEX")
                .about("specify the pattern to use for extraction")
                .help_heading(Some("Extractors")),
        )
        .group(
            ArgGroup::new("pattern")
                .args(&["phone", "email", "regex"])
                .multiple(true)
                .required(true),
        )
        .arg(
            Arg::new("data-only")
                .long("data-only")
                .short('o')
                .takes_value(false)
                .about("only print the extracted data, without the source url")
                .help_heading(Some("Output")),
        )
        .arg(
            Arg::new("json")
                .long("json")
                .short('j')
                .takes_value(false)
                .about("print the data as json")
                .help_heading(Some("Output")),
        )
        .arg(
            Arg::new("pretty-json")
                .long("pretty-json")
                .takes_value(false)
                .about("print the data as pretty json")
                .help_heading(Some("Output")),
        )
        .arg(
            Arg::new("full")
                .long("full")
                .short('f')
                .takes_value(false)
                .about("print the url as a heading before the found data (default)")
                .help_heading(Some("Output")),
        )
        .group(ArgGroup::new("output").args(&["data-only", "full", "json", "pretty-json"]))
        .get_matches();

    // extract the values needed for traversal
    let max_depth: Option<usize> = match app.value_of("max-depth") {
        Some(depth_s) => match usize::from_str(depth_s) {
            Ok(depth) => Some(depth),
            _ => {
                eprintln!("ERROR: unable to parse depth as uint");
                exit(1);
            }
        }
        None => None
    };

    let root: Url = match Url::parse(app.value_of("root").unwrap()) {
        Ok(url) => url,
        Err(err) => {
            eprintln!("ERROR: {}", err.to_string());
            exit(1);
        }
    };

    // get the matching pattern
    let mut patterns: Vec<String> = vec![];

    if app.is_present("phone") {
        patterns.push(PatternType::get_regexp(PatternType::Phone));
    }

    if app.is_present("email") {
        patterns.push(PatternType::get_regexp(PatternType::Email));
    }

    if app.is_present("regex") {
        patterns.push(PatternType::get_regexp(PatternType::Regex(
            app.value_of("regex").unwrap(),
        )));
    }

    // extract data from site
    let patterns: Vec<&str> = patterns.iter().map(String::as_str).collect();
    let regexp: Regex = match Regex::new(&patterns.join("|")) {
        Ok(r) => r,
        _ => {
            eprintln!("ERROR: invalid regex pattern");
            exit(1);
        }
    };

    let graph: Graph = match Graph::new(root, &regexp, max_depth) {
        Some(g) => g,
        None => {
            exit(1);
        }
    };

    // output the extracted dta in the requested format
    if app.is_present("data-only") {
        for (_, node) in graph.iter() {
            for datum in &node.data {
                println!("{}", datum);
            }
        }
    } else if app.is_present("json") {
        let json: String = if let Ok(j) = serde_json::to_string(&graph) {
            j
        } else {
            eprintln!("ERROR: Could not generate json output.");
            return;
        };
        println!("{}", json);
    } else if app.is_present("pretty-json") {
        let json: String = if let Ok(j) = serde_json::to_string_pretty(&graph) {
            j
        } else {
            eprintln!("ERROR: Could not generate json output.");
            return;
        };
        println!("{}", json);
    } else {
        for (url, node) in graph.iter() {
            println!("{}", url.as_str());
            for datum in &node.data {
                println!("├─ {}", datum);
            }
        }
    }
}

mod extract;
mod node;

use std::str::FromStr;

use regex::Regex;

use clap::{
    crate_authors, crate_description, crate_name, crate_version, App, Arg, ArgGroup, ArgMatches,
};

use url::Url;

use extract::{Graph, PatternType};

// todo: better output
//   json?
//   flat &| mapped
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
                .help_heading(Some("Output"))
        )
        .arg(
        Arg::new("json")
            .long("json")
            .short('j')
            .takes_value(false)
            .about("print the data as a json value")
            .help_heading(Some("Output"))
        )
        .arg(
            Arg::new("full")
                .long("full")
                .short('f')
                .takes_value(false)
                .about("print the url as a heading before the found data (default)")
                .help_heading(Some("Output"))
        )
        .group(
            ArgGroup::new("output")
                .args(&["data-only", "full", "json"])
        )
        .get_matches();

    // extract the values needed for traversal
    let max_depth: Option<usize> = if let Some(depth_s) = app.value_of("max-depth") {
        if let Ok(depth) = usize::from_str(depth_s) {
            Some(depth)
        } else {
            eprintln!("ERROR: Unable to parse depth as uint\nsee `{} --help` for more information.", crate_name!());
            return;
        }
    } else {
        None
    };

    let root: Url = match Url::parse(app.value_of("root").unwrap()) {
        Ok(url) => url,
        Err(err) => {
            eprintln!("{}", err.to_string());
            return;
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
    let regexp: Regex = Regex::new(&patterns.join("|")).unwrap();
    let graph: Graph = Graph::new(root, &regexp, max_depth);

    // output the extracted dta in the requested format
    if app.is_present("data-only") {
        for (_, node) in graph.iter() {
            for datum in &node.data {
                println!("{}", datum);
            }
        }
    }
    else if app.is_present("json") {
        let json: String = if let Ok(j) = serde_json::to_string(&graph) {
            j
        } else {
            eprintln!("ERROR: Could not generate json output.");
            return;
        };
        println!("{}", json);
    }
    else {
        // todo: could make this cleaner with more tree-like output for last datum
        for (url, node) in graph.iter() {
            println!("{}", url.as_str());
            for datum in &node.data {
                println!("├─ {}", datum);
            }
        }
    }
}

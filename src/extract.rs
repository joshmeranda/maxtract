use std::collections::HashSet;
use std::str;
use std::string::String;

use curl::easy::{Easy2, Handler, WriteError};
use regex::{Match, Regex};

use select::document::Document;
use select::node::Node as DomNode;
use select::predicate::Name;

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

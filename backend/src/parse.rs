use pest::Parser;
use pest_derive::Parser;
use std::fs::File;
use std::io::{Error,Write};

#[derive(Parser)]
#[grammar = "grammar.pest"] 
pub struct MarkupParser;

pub fn parse(source: String, pagename: String,pagedir: &String) {
    let htmlpath = format!("{}/{}/{}html.html",pagedir,pagename,pagename);
    let mut reshtml: String =  format!(
r#"<!DOCTYPE html>
<head title="{pagename}"
<body>
<h1>{pagename}<h1>
<hr><br>"#
);
    let result = MarkupParser::parse(Rule::start, source.as_str());
    match result {
        Ok(pairs) => {
            for pair in pairs {
                traverse(pair,reshtml);
            }
        }
        Err(e) => {
            eprintln!("Parsing error: {}", e);
        }
    }
}

fn traverse(pair: pest::iterators::Pair<Rule>, reshtml: String) {
    match pair.as_rule() {
        Rule::start => {}
        Rule::NEWLINE => {reshtml.push("
        ")}
        Rule::psstart => {}
    }
    println!("{:indent$}{:?} => {}", "", pair.as_rule(), pair.as_str());
    for inner in pair.into_inner() {
        traverse(inner);
    }
}

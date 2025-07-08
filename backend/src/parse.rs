use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"] 
pub struct MarkupParser;

pub fn parse() {
    let source = r#"[# phead]__Hello__ is a word. [br]`print("hello world");`[#/phead][! infobox][title [:] xyz][img [:] xyz][!/ infobox]"#;
    let result = MarkupParser::parse(Rule::start, source);

    match result {
        Ok(pairs) => {
            for pair in pairs {
                traverse(pair, 0);
            }
        }
        Err(e) => {
            eprintln!("Parsing error: {}", e);
        }
    }
}

fn traverse(pair: pest::iterators::Pair<Rule>, indent: usize) {
    println!("{:indent$}Rule: {:?} => {}", "", pair.as_rule(), pair.as_str(), indent = indent * 2);
    for inner in pair.into_inner() {
        traverse(inner, indent + 1);
    }
}

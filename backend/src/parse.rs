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
                reshtml = traverse(pair,reshtml);
            }
        }
        Err(e) => {
            println!("Parsing error: {}", e);
        }
    }
    reshtml.push_str("</body>");
    println!("{}",reshtml);
}

fn traverse(pair: pest::iterators::Pair<Rule>, reshtml: String) -> String{
    let mut reshtml = reshtml;
    match pair.as_rule() {
        Rule::start => {}
        Rule::WHITESPACE => {}
        Rule::CNAME => {}
        Rule::code => {}
        Rule::text => {}
        Rule::textpart => {}
        Rule::section => {}
        Rule::pagesection => {}
        Rule::boxsection => {}
        Rule::boxcontent => {}
        Rule::keyvalue => {}
        Rule::key => {}
        Rule::value => {}
        Rule::std => {reshtml.push_str(format!("<p>{}</p>\n",pair.as_str()).as_str());}
        Rule::quote => {
            let mut content = String::from("");
            for inner in pair.clone().into_inner() {
                if inner.as_rule() == Rule::CNAME {
                    content = String::from(inner.as_str());
                    break;
                }
            }
            reshtml.push_str(format!("<q>{}</q>\n",content).as_str());
        }
        Rule::bold => {reshtml.push_str(format!("<b>{}</b>\n",pair.as_str().replace("__", "")).as_str());}
        Rule::italic => {reshtml.push_str(format!("<i>{}</i>\n",pair.as_str().replace("_", "")).as_str());}
        Rule::NEWLINE => {reshtml.push_str("<br>");}
        Rule::psstart => {
            let mut id = String::new();
            for inner in pair.clone().into_inner() {
                if inner.as_rule() == Rule::CNAME {
                    id = String::from(inner.as_str());
                    break;
                }
            }
            reshtml.push_str(format!("<div id={}><h2>{}</h2><hr><br>\n",id,id).as_str());
        }
        Rule::psend => {
            let mut id = String::from("");
            for inner in pair.clone().into_inner() {
                if inner.as_rule() == Rule::CNAME {
                    id = String::from(inner.as_str());
                    break;
                }
            }
            if id != String::from("") {
                reshtml.push_str(format!("</div id={}><br>\n",id).as_str());
            } else {
                reshtml.push_str("</div><br>\n");
            }
        }
        Rule::boxstart => {
            let mut id = String::new();
            for inner in pair.clone().into_inner() {
                if inner.as_rule() == Rule::CNAME {
                    id = String::from(inner.as_str());
                    break;
                }
            }
            reshtml.push_str(format!("<div id={}><h2>{}</h2><hr><br>\n",id,id).as_str());
        }
        Rule::boxend => {
            let mut id = String::from("");
            for inner in pair.clone().into_inner() {
                if inner.as_rule() == Rule::CNAME {
                    id = String::from(inner.as_str());
                    break;
                }
            }
            if id != String::from("") {
                reshtml.push_str(format!("</div id={}><br>\n",id).as_str());
            } else {
                reshtml.push_str("</div><br>\n");
            }
        }
        Rule::codetext => {reshtml.push_str(format!("<code>{}</code>\n",pair.as_str()).as_str());} 
    }
    let indent = 0;
    println!("{:indent$}{:?} => {}", "", pair.as_rule(), pair.as_str());
    for inner in pair.into_inner() {
        reshtml = traverse(inner, reshtml);
    }
    return reshtml;
}

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"] 
pub struct MarkupParser;

pub fn parse(source: String, pagename: &String) -> String {
    let mut reshtml: String =  format!(
r#"<!DOCTYPE html>
<head>
<title>
{pagename}
</title>
</head>
<body>
<style>
h1 {{
  font-size: 40px;
  font-family: 'Linux Libertine','Georgia','Times','Source Serif Pro',serif;
  font-weight: 600;
  margin-bottom: 0.1%;
    }}

h2 {{
font-size: 30px;
    }}
hr {{
  margin-bottom: 1px;
    }}
body {{
  font-size: 16px;
}}
</style>
<h1>{pagename}</h1>
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
    return reshtml;
}

fn traverse(pair: pest::iterators::Pair<Rule>, reshtml: String) -> String{
    let mut reshtml = reshtml;
    let mut processedinner = false;
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
        Rule::std => {reshtml.push_str(format!("{} ",pair.as_str()).as_str());}
        Rule::quote => {
            let mut content = String::from("");
            for inner in pair.clone().into_inner() {
                content = traverse(inner, content);
            }
            processedinner = true;
            reshtml.push_str(format!("<q>{}</q>",content).as_str());
        }
        Rule::bold => {
            let mut content = String::from("");
            for inner in pair.clone().into_inner() {
                content = traverse(inner, content);
            }
            processedinner = true;

            reshtml.push_str(format!("<b>{}</b>",content).as_str());
        }
        Rule::italic => {
            let mut content = String::from("");
            for inner in pair.clone().into_inner() {
                content = traverse(inner, content);
            }
            processedinner = true;

            reshtml.push_str(format!("<i>{}</i>",content).as_str());
        }
        Rule::NEWLINE => {reshtml.push_str("<br>");}
        Rule::psstart => {
            let mut id = String::new();
            for inner in pair.clone().into_inner() {
                if inner.as_rule() == Rule::CNAME {
                    id = String::from(inner.as_str());
                    break;
                }
            }
            if id != String::from("phead") {
                processedinner = true;
                reshtml.push_str(format!("<div id={}><h2>{}</h2><hr><br>\n",id,id).as_str());
            } else {
                processedinner = false;
            }
        }
        Rule::psend => {
            let mut id = String::from("");
            for inner in pair.clone().into_inner() {
                if inner.as_rule() == Rule::CNAME {
                    id = String::from(inner.as_str());
                    break;
                }
            }
            processedinner = true;
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
            processedinner = true;
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
            processedinner = true;
            if id != String::from("") {
                reshtml.push_str(format!("</div id={}><br>\n",id).as_str());
            } else {
                reshtml.push_str("</div><br>\n");
            }
        }
        Rule::codetext => {reshtml.push_str(format!("<code>{}</code>\n",pair.as_str()).as_str());} 
    }
    if !processedinner {
        for inner in pair.into_inner() {
            reshtml = traverse(inner, reshtml);
    }
}
    return reshtml;
}

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"] 
pub struct MarkupParser;

pub fn parse(source: String, pagename: &String) -> (String,String) {
    let mut plaintext = String::new();
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
                (reshtml,plaintext) = traverse(pair,reshtml,plaintext);
            }
        }
        Err(e) => {
            println!("Parsing error: {}", e);
        }
    }
    reshtml.push_str("</body>");
    return (reshtml,plaintext);
}

fn traverse(pair: pest::iterators::Pair<Rule>, reshtml: String,plaintext: String) -> (String,String){
    let mut reshtml = reshtml;
    let mut plaintext = plaintext;
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
        Rule::std => {
            reshtml.push_str(format!("{} ",pair.as_str()).as_str());
            plaintext.push_str(format!("{}",pair.as_str()).as_str());
        }
        Rule::quote => {
            let mut contenthtml = String::from("");
            let mut contentplaintext = String::new();
            for inner in pair.clone().into_inner() {
                (contenthtml,contentplaintext) = traverse(inner, contenthtml,contentplaintext);
            }
            processedinner = true;
            reshtml.push_str(format!("<q>{}</q>",contenthtml).as_str());
            reshtml.push_str(format!("{}",contentplaintext).as_str());

        }
        Rule::bold => {
            let mut contenthtml = String::from("");
            let mut contentplaintext = String::new();
            for inner in pair.clone().into_inner() {
                (contenthtml,contentplaintext) = traverse(inner, contenthtml,contentplaintext);
            }
            processedinner = true;
            reshtml.push_str(format!("<b>{}</b>",contenthtml).as_str());
            reshtml.push_str(format!("{}",contentplaintext).as_str());

        }
        Rule::italic => {
            let mut contenthtml = String::from("");
            let mut contentplaintext = String::new();
            for inner in pair.clone().into_inner() {
                (contenthtml,contentplaintext) = traverse(inner, contenthtml,contentplaintext);
            }
            processedinner = true;
            reshtml.push_str(format!("<i>{}</i>",contenthtml).as_str());
            reshtml.push_str(format!("{}",contentplaintext).as_str());

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
                plaintext.push_str(format!("{}",id).as_str());
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
            plaintext.push_str(format!("{}",id).as_str());
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
        Rule::codetext => {
            reshtml.push_str(format!("<code>{}</code>\n",pair.as_str()).as_str());
            plaintext.push_str(format!("{}",pair.as_str()).as_str());
        } 
    }
    if !processedinner {
        for inner in pair.into_inner() {
            (reshtml,plaintext) = traverse(inner, reshtml,plaintext);
    }
}
    return (reshtml,plaintext);
}

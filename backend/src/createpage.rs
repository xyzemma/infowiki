use std::fs::File;
use std::io::prelude::*;
use std::time::{SystemTime,UNIX_EPOCH};
use crate::parse;
use crate::init::Page;
use rusqlite::{Connection,params,Result};

pub enum CrpResp {
    Success,
    Error(String)
}
pub fn create_page(name: String,text:String) -> CrpResp {
    let conn: Connection = Connection::open("db.db3").expect("ERROR");
    match std::fs::create_dir_all(format!("pages/{}",name)) {
        Ok(_) => {}
        Err(error) => {
            return CrpResp::Error(String::from(format!("{}",error)))
        }
    }
    let mut wtfile = match File::create(format!("pages/{}/{}markdown.md",name,name)) {
        Ok(result) => {result}
        Err(error) => {
            return CrpResp::Error(String::from(format!("{}",error)))
        }
    };
    let mut htmlfile = match File::create(format!("pages/{}/{}html.html",name,name)) {
        Ok(result) => {result}
        Err(error) => {
            return CrpResp::Error(String::from(format!("{}",error)))
        }
    };
    let mdtext: String = format!("{}",text);
    match wtfile.write_all(mdtext.as_bytes()) {
        Ok(_) => {
        }
        Err(error) => {
            return CrpResp::Error(String::from(format!("Failed to create page '{}': {}",name,error)));
        }
    }
    let (htmltext, plaintext) = parse::parse(mdtext,&name);
    let pagesql = Page {
        id: 0,
        name: name.to_string(),
        created_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        text: plaintext.to_string(),
    };
    conn.execute("INSERT INTO page (name,text,created_at) VALUES (?1,?2,?3)", (&pagesql.name,&pagesql.text,&pagesql.created_at)).expect("ERROR");
    match htmlfile.write_all(htmltext.as_bytes()) {
        Ok(_) => {
            return CrpResp::Success;
        }
        Err(error) => {
            return CrpResp::Error(String::from(format!("Failed to create page '{}': {}",name,error)));
        }
    }
}
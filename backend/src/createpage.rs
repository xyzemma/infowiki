use std::fs::File;
use std::io::prelude::*;
use std::time::{SystemTime,UNIX_EPOCH};
use crate::parse;
use crate::init::Page;
use rusqlite::ffi::Error;
use rusqlite::{Connection,params,Result};
use crate::init::{IwResp,IwError};

pub fn create_page(name: String,text:String) -> IwResp {
    let conn: Connection = match Connection::open("db.db3") {
        Ok(conn) => {conn}
        Err(error) => {
            return IwResp::Error(IwError {errormsg: format!("{}",error),timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()});
        }
    };
    match std::fs::create_dir_all(format!("pages/{}",name)) {
        Ok(_) => {}
        Err(error) => {
            return IwResp::Error(IwError {errormsg: String::from(format!("{}",error)),timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()})
        }
    }
    let mut wtfile = match File::create(format!("pages/{}/{}markdown.md",name,name)) {
        Ok(result) => {result}
        Err(error) => {
            return IwResp::Error(IwError {errormsg: String::from(format!("{}",error)),timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()})
        }
    };
    let mut htmlfile = match File::create(format!("pages/{}/{}html.html",name,name)) {
        Ok(result) => {result}
        Err(error) => {
            return IwResp::Error(IwError {errormsg: String::from(format!("{}",error)), timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()})
        }
    };
    let mdtext: String = format!("{}",text);
    match wtfile.write_all(mdtext.as_bytes()) {
        Ok(_) => {
        }
        Err(error) => {
            return IwResp::Error(IwError {errormsg: String::from(format!("Failed to create page '{}': {}",name,error)),timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()});
        }
    }
    let (htmltext, plaintext) = parse::parse(mdtext,&name);
    let pagesql = Page {
        id: 0,
        name: name.to_string(),
        created_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        text: plaintext.to_string(),
        current_version: 0,
    };
    conn.execute("INSERT INTO page (name,text,created_at) VALUES (?1,?2,?3)", (&pagesql.name,&pagesql.text,&pagesql.created_at)).expect("ERROR");
    match htmlfile.write_all(htmltext.as_bytes()) {
        Ok(_) => {
            return IwResp::Success;
        }
        Err(error) => {
            return IwResp::Error(IwError {errormsg: String::from(format!("Failed to create page '{}': {}",name,error)), timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()});
        }
    }
}
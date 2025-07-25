use core::panic;
use std::fs::read_to_string;
use serde_json;
use rusqlite::{params,Connection,Result};
use git2::Repository;

pub enum IwResp {
    Success,
    Error(String)
}

#[derive(Debug)]
pub struct Page {
    pub id: u32,
    pub name: String,
    pub created_at: u64,
    pub text: String,
}

pub async fn init() -> (String, String, Repository) {
    let repo = match Repository::open("pages") {
        Ok(repo) => repo,
        Err(_) => {
            match Repository::init("pages") {
                Ok(repo) => repo,
                Err(e) => panic!("Error initialising git repo: {}",e)
            }
        } 
    };
    match dbinit() {
        Ok(_) => {},
        Err(e) => panic!("Error initialising database: {}",e)
    };
    let config = match read_to_string("config.json") {
        Ok(val) => {val}
        Err(err) => {
            println!("Error initialising Infowiki: Config file could not be read: {}",err);
            panic!()}
    };
    let configjson: serde_json::Value = match serde_json::from_str(config.as_str()) {
        Ok(val) => {val}
        Err(err) => {
        println!("Error initialising Infowiki: Config file could not be processed: {}",err);
            panic!()}
    };
    let mainpath: String = format!("{}",configjson["path"]);
    let pagepath = format!("{}/pages",mainpath);
    if std::path::Path::new(&pagepath).exists() != true {
        match std::fs::create_dir_all(format!("pages")) {
            Ok(_) => {}
            Err(error) => {
                println!("{}",error)
            }
        }
    }
    return (mainpath,pagepath,repo)
}

pub fn dbinit() -> Result<()> {
    let conn: Connection = Connection::open("db.db3")?;
    match conn.execute("SELECT name FROM sqlite_master WHERE type='table' AND name='page'",()) {
        Ok(_) => {
            match conn.execute("CREATE TABLE page (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                created_at INTEGER,
                text TEXT NOT NULL
                )", ()) {
                    Ok(_) => {},
                    Err(e) => {return Err(e);}
                }
        },
        Err(_) => {}
        
    }
    
    let mut stmt = conn.prepare("SELECT id, name, created_at, text FROM page")?;
    let page_iter = stmt.query_map([], |row| {
        Ok(Page {
            id: row.get(0)?,
            name: row.get(1)?,
            created_at: row.get(2)?,
            text: row.get(3)?,
        })
    })?;

    for page in page_iter {
        println!("Found page {:?}", page?);
    }
    Ok(())
}
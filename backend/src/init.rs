use core::panic;
use std::fs::read_to_string;
use std::fs::File;
use serde_json;
use rusqlite::{params,Connection,Result};
use crate::errorhandling;
use crate::{errorhandling::{IwError,IwResp}, main};
use std::time::{SystemTime,UNIX_EPOCH};


#[derive(Debug)]
pub struct Page {
    pub id: u32,
    pub name: String,
    pub created_at: u64,
    pub text: String,
    pub current_version: u64,
}


pub async fn init() -> (String,String,String,File) {
    let (mainpath,pagepath) = match get_paths() {
        Ok(val) => val,
        Err(e) => {
            panic!("{}",e.errormsg)
        }
    };
    match dbinit(&mainpath) {
        Ok(_) => {},
        Err(e) => panic!("Error initialising database: {}",e)
    };

    let pid: String = format!("iw_{}",SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());
    let mut logfile: File;
    match File::create(format!("{}/logs",mainpath)) {
        Ok(v) => {
            logfile = v;
        },
        Err(e) => {
            panic!("{}",e)
        }
    }
    return (mainpath,pagepath,pid,logfile)
}


pub fn get_paths() -> Result<(String,String),IwError> {
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
    let mainpath: String = format!("{}",configjson["path"]).replace('"', "");
    let pagepath = format!("{}/pages",mainpath);
    if std::path::Path::new(&pagepath).exists() != true {
        match std::fs::create_dir_all(format!("pages")) {
            Ok(_) => {}
            Err(error) => {
                println!("{}",error)
            }
        }
    }
    Ok((mainpath,pagepath))
}

pub fn dbinit(path: &String) -> Result<()> {
    let conn: Connection = Connection::open(format!("{}/db.db3",path.as_str()))?;
    match conn.execute("SELECT name FROM sqlite_master WHERE type='table' AND name='page'",()) {
        Ok(_) => {
            match conn.execute("CREATE TABLE page (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                created_at INTEGER,
                text TEXT NOT NULL,
                currentversion INTEGER
                )", ()) {
                    Ok(_) => {},
                    Err(e) => {return Err(e);}
                }
        },
        Err(_) => {}
        
    }
    match conn.execute("SELECT name FROM sqlite_master WHERE type='table' AND name='versions'",()) {
        Ok(_) => {
            match conn.execute("CREATE TABLE versions (
                id INTEGER PRIMARY KEY,
                page TEXT NOT NULL,
                author INTEGER,
                versionnum INTEGER,
                timestamp INTEGER,
                diff TEXT NOT NULL
                )", ()) {
                    Ok(_) => {},
                    Err(e) => {return Err(e);}
                }
        },
        Err(_) => {}
        
    }
    
    let mut stmt = conn.prepare("SELECT id, name, created_at, text, currentversion FROM page")?;
    let page_iter = stmt.query_map([], |row| {
        Ok(Page {
            id: row.get(0)?,
            name: row.get(1)?,
            created_at: row.get(2)?,
            text: row.get(3)?,
            current_version: row.get(4)?,
        })
    })?;

    for page in page_iter {
        println!("Found page {:?}", page?);
    }
    Ok(())
}
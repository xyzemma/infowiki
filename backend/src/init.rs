use std::fs::read_to_string;
use serde_json;
use rusqlite::{params,Connection,Result};
use std::time::{SystemTime,UNIX_EPOCH};

#[derive(Debug)]
struct Page {
    id: u32,
    name: String,
    created_at: u64,
    text: String,
}

pub async fn init() -> (String, String) {
    dbinit();
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
    return (mainpath,pagepath)
}

pub fn dbinit() -> Result<()> {
    let conn: Connection = Connection::open("db.db3")?;
    match conn.execute("CREATE TABLE page (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    created_at INTEGER,
    text TEXT NOT NULL
    )", ()) {
        Ok(_) => {},
        Err(e) => {return Err(e);}
    }
    let testpage = Page {
        id: 0,
        name: "test".to_string(),
        created_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        text: "Hello World".to_string(),
    };
    conn.execute("INSERT INTO page (name,text,created_at) VALUES (?1,?2,?3)", (&testpage.name,&testpage.text,&testpage.created_at))?;
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
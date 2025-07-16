use std::fs::read_to_string;
use serde_json;
use dotenv::dotenv;
use postgres::{Client,NoTls};


pub fn init() -> (String, String) {
    dotenv().ok();
    let dbhost = std::env::var("DBHOST").expect("DBHOST must be set.");
    let dbuser = std::env::var("DBUSER").expect("DBUSER must be set");
    let mut dbclient = Client::connect(format!("host={dbhost} user={dbuser}").as_str(), NoTls);
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
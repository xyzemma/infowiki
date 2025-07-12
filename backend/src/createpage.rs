use std::fs::File;
use std::io::prelude::*;

use crate::parse;

pub enum CrpResp {
    Success,
    Error(String)
}
pub fn create_page(name: String,text:String) -> CrpResp {
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
    let htmltext:String = parse::parse(mdtext,&name);
    match htmlfile.write_all(htmltext.as_bytes()) {
        Ok(_) => {
            return CrpResp::Success;
        }
        Err(error) => {
            return CrpResp::Error(String::from(format!("Failed to create page '{}': {}",name,error)));
        }
    }
}
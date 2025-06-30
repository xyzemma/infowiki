use std::fs::File;
use std::io::prelude::*;
pub enum CrpResp {
    Success,
    Error(String)
}
pub fn create_page(name: String,text:String) -> CrpResp {
    match std::fs::create_dir_all(format!("{}",name)) {
        Ok(_) => {}
        Err(error) => {
            return CrpResp::Error(String::from(format!("{}",error)))
        }
    }
    let mut file = match File::create(format!("{}/{}markdown.md",name,name)) {
        Ok(result) => {result}
        Err(error) => {
            return CrpResp::Error(String::from(format!("{}",error)))
        }
    };
    let text: String = format!("{}",text);
    match file.write_all(text.as_bytes()) {
        Ok(_) => {
            return CrpResp::Success;
        }
        Err(error) => {
            return CrpResp::Error(String::from(format!("Failed to create page '{}': {}",name,error)));
        }
    }
}
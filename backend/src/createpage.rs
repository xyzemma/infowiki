use std::fs::File;
use std::io::prelude::*;
pub fn create_page(name: String,text:String) -> bool {
    std::fs::create_dir_all(format!("{}",name));
    let mut file = File::create(format!("{}/{}markdown.md",name,name));
    let text: String = format!("{}",text);
    match file.expect("REASON").write_all(text.as_bytes()) {
        Ok(_) => {
            return true;
        }
        Err(error) => {
            return false;
        }
    }
}
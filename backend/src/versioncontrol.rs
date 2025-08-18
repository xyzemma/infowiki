use crate::init::Page;
use crate::users;

pub struct Line {
    content: String,
    linenumber: u64,
}
pub struct Diff {
    added: Vec<Line>,
    removed: Vec<Line>,
}

pub struct Version {
    id: u64,
    diff: Diff,
    timestamp: u64,
    author: users::User,
} 

pub fn lineify(txt: String) -> Vec<Line> {

    for i in txt.lines() {
        println!("{}",i);

    }
    return Vec::new();
}
pub fn commit(){
}
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
    let mut rvec: Vec<Line> = Vec::new();
    let mut ln: u64 = 1;
    for i in txt.lines() {
        rvec.push(Line { content: String::from(i), linenumber: ln });
        ln += 1;
    }
    return rvec;
}
pub fn commit(){
}
use crate::init::Page;
use crate::users;

#[derive(Clone)]
pub struct Line {
    pub content: String,
    pub linenumber: u64,
}
pub struct Diff {
    pub added: Vec<Line>,
    pub removed: Vec<Line>,
}

pub struct Version {
    pub id: u64,
    pub diff: Diff,
    pub timestamp: u64,
    pub author: users::User,
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

pub fn diff(old: Vec<Line>,new: Vec<Line>) -> Diff {
    let mut all: Vec<Line> = Vec::new();
    let mut added: Vec<Line> = Vec::new();
    let mut removed: Vec<Line> = Vec::new();
    all.extend(old.clone());
    all.extend(new.clone());
    for i in all {
        if new.iter().any(|s| s.content == i.content) && !old.iter().any(|s| s.content == i.content) {
            added.push(i.clone())
        }
        if !new.iter().any(|s| s.content == i.content) && old.iter().any(|s| s.content == i.content) {
            removed.push(i)
        }
    }
    return Diff {
        added: added,
        removed: removed,
    }
}

pub fn commit(){
}
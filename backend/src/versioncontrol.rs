use crate::init::Page;
use crate::users;
use actix_web::Error;
use rusqlite::{params,Connection,Result as sqlres,Error as sqlerr};
use serde_json::{Result as sjr,Value,Error as serr};
use serde::{Deserialize, Serialize};
use serde_derive::{Deserialize,Serialize};

pub enum gv_resp {
    Ok(Vec<Version>),
    Err(String)
}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Line {
    pub content: String,
    pub linenumber: u64,
}
#[derive(Debug,Deserialize,Serialize)]
pub struct Diff {
    pub added: Vec<Line>,
    pub removed: Vec<Line>,
}

#[derive(Debug)]
pub struct Version {
    pub id: u64,
    pub diff: Diff,
    pub timestamp: u64,
    pub author: String,
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

pub fn get_versions(p: Page,path: String) -> Result<gv_resp,sqlerr> {
    let mut rvec: Vec<Version> = Vec::new();
    let conn: Connection = Connection::open(format!("{}/db.db3",path.as_str()))?;
    let mut stmt = conn.prepare("SELECT versionnum, author, timestamp, diff FROM page")?;
    let v_iter = stmt.query_map([], |row| {
        Ok(Version {
            id: row.get(0)?,
            author: row.get(1)?,
            timestamp: row.get(2)?,
            diff: match row.get(3) {
                Ok(v) => {deserialize_diff(v).unwrap()}
                Err(_) => {return Err(sqlerr::QueryReturnedNoRows)},
            }
        })
    })?;

    for v in v_iter {
        rvec.push(v.unwrap());
    }
    return Ok(gv_resp::Ok(rvec));
}

pub fn deserialize_diff(input: String) -> Result<Diff,serr> {
    let diff: Diff = serde_json::from_str(input.as_str())?;
    Ok(diff)
}

//pub fn commit(p: Page,newtext:String) -> (Page,Version) {
//}
use std::fs::OpenOptions;
use std::io::prelude::*;

use crate::init;

pub enum IwResp {
    Success,
    Error(IwError)
}

#[derive(Debug)]
pub struct IwError {
    pub errormsg: String,
    pub timestamp: u64,
}

pub fn raise_and_log(error: IwError) -> IwError {
    let (path,_) = init::get_paths().unwrap();
    path.push_str("/logs");
    let mut logfile 
}

pub fn log_before_panic(error: IwError,pathfound: bool) {
    
}
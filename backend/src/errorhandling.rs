pub enum IwResp {
    Success,
    Error(IwError)
}

pub struct IwError {
    pub errormsg: String,
    pub timestamp: u64,
}
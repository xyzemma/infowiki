use crate::init::Page;
mod users;

pub struct Version {
    page: Page,
    timestamp: u64,
    author: users::User,
} 

pub fn commit() -> Version {
    
}
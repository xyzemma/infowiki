use uuid::Uuid;

#[derive(Debug)]
pub struct User {
    pub uuid: Uuid,
    pub name: String,
}

use serde_derive::Serialize;
use serde_derive::Deserialize;

#[derive(Serialize,Deserialize, Debug)]
pub enum UserRole {
    BASIC,
    ADMIN,
}

#[derive(Serialize,Deserialize, Debug)]
pub struct User {
    pub name: String,
    pub email: String,
    pub age: u16,
    pub active: bool,
    pub role: UserRole,
    pub description: Option<String>
}


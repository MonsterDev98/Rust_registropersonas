pub enum UserRole {
    BASIC,
    ADMIN,
}

pub struct User {
    pub name: String,
    pub email: String,
    pub age: u16,
    pub active: bool,
    pub role: UserRole,
    pub description: Option<String>
}
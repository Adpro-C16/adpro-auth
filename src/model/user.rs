use std::fmt;

use serde::Serialize;

#[derive(Serialize)]
pub enum Role {
    AdminUser,
    User,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Role::AdminUser => write!(f, "ADMIN"),
            Role::User => write!(f, "USER"),
        }
    }
}
#[derive(Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub role: Role,
    pub balance: i32,
}

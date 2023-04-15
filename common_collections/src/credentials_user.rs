// creating a user struct to hold user name and password
use std::collections::HashMap;
pub struct User {
    pub username: String,
    pub password: String,
}
#[derive(Debug)]
pub struct UserStatus {
    pub active: bool,
    pub login_time: i32,
    pub name: String,
}
//return the user info

impl UserStatus {
    pub fn return_status(x_name: &String) -> UserStatus {
        UserStatus {
            active: true,
            login_time: 1,
            name: x_name.to_string(),
        }
    }
}

// creating hash map of user

pub fn creat_user() -> HashMap<String, User> {
    let mut users = HashMap::new();
    users.insert(
        String::from("Twariq"),
        User {
            username: String::from("Twariq"),
            password: String::from("123456"),
        },
    );
    users.insert(
        String::from("sami"),
        User {
            username: String::from("sami"),
            password: String::from("123456"),
        },
    );
    users
}

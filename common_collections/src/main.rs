mod credentials_user;
use std::io::{self, Write};
fn main() {
    //creating the instance of Hash map
    let users_list = credentials_user::creat_user();

    println!("enter the user name");
    io::stdout().flush().unwrap();
    let mut uname = String::new();
    io::stdin().read_line(&mut uname).expect("failed to read");
    let uname = uname.trim();

    println!("enter the password");
    io::stdout().flush().unwrap();
    let mut upwd = String::new();
    io::stdin().read_line(&mut upwd).expect("failed to read");
    let upwd = upwd.trim();

    if let Some(userv) = users_list.get(uname) {
        if userv.password == upwd {
            println!(
                "{:?}",
                credentials_user::UserStatus::return_status(&uname.to_string())
            );
        }
    } else {
        println!("Incorrect username or password. Please try again.");
    }
}

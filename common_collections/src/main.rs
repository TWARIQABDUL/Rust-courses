use std::io;

use credentials_user::User;
mod credentials_user;


fn main() {
    println!("please login into the system");
    let mut user_name = String::new();
    let mut password = String::new();
    println!("type user name");
    io::stdin().read_line(&mut user_name).expect("something is wrong");
    println!("type user password");
    io::stdin().read_line(&mut password).expect("something is wrong");
    let ff = User::hell();
    println!("{}",ff.uname);
}

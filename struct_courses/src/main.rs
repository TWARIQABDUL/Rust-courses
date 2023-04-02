struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_acount: u32,
}
#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}
impl Rect {
    fn calc_area(&self) -> u32 {
        self.width * self.height
    }
}
fn main() {
    // this is how to initialize a struct

    let mut user1 = User {
        active: true,
        username: String::from("twariq abdalazizi"),
        email: String::from("twariq@gmail.com"),
        sign_in_acount: 1,
    };

    let user2 = User {
        email: String::from("new user@gmail.com"),
        ..user1
    };
    println!(
        "user2 email {} has also name {}",
        user1.email, user2.username
    );
    let rect1 = Rect {
        height: 30,
        width: 20,
    };

    println!("the area of rectangle = {}", rect1.calc_area());
    dbg!(&rect1);
}

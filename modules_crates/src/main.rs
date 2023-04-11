mod back_of_house;
use std::io;
fn main() {
    back_of_house::serving::greet_customer();
    let mut order = String::new();
    println!("Please choose an item from the menu:");
    io::stdin().read_line(&mut order).expect("Something went wrong");
    back_of_house::serving::get_order(&order);

}

use std::io;
fn main() {
    println!("Hello, world welcome to compare !");
    println!("enter the first number");
    let arr = [1,2,3,4,5];
    let mut n1 = String::new();
    io::stdin()
        .read_line(&mut n1)
        .expect("something went wrong");
    println!("enter the second number");
    let mut n2 = String::new();
    io::stdin()
        .read_line(&mut n2)
        .expect("something went wrong");
    let sum = calc_(n1.trim().to_string(), n2.trim().to_string());
    if sum > 0 {
        println!(" {} + {} = {}", n1, n2, sum);
    } else {
        println!("the answer i zero");
    }

    // for loop
    for elem in arr{
        println!("the number😊 {}",elem);
    }


}
fn convert(n: String) -> u32 {
    let newn: u32 = n.trim().parse().expect("not a number");
    newn
}
fn calc_(v1: String, v2: String) -> u32 {
    let n1 = convert(v1);
    let n2 = convert(v2);
    n1 + n2
}

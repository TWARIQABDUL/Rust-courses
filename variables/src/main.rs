use std::io;
fn main() {
    const PI:f32 = 3.14;
    let mut n1 = String::new();
    let mut n2 = String::new();

    io::stdin().read_line(&mut n1).expect("sonething went wrong");
    io::stdin().read_line(&mut n2).expect("sonething went wrong");

    let n1:u32 = n1.trim().parse().expect("not a number");
    let n2:u32 = n2.trim().parse().expect("not a number");

    // add 
    let add = n1 + n2;
    print!("{} + {} = {}",n1,n2,add);

    // multi 
    let mult = n1 * n2;
    print!("{} * {} = {}",n1,n2,mult);



}

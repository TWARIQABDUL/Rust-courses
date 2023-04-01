use std::io;
fn main() {
    let mut s = String::from("hello");
    let mut strng = String::new();

    io::stdin()
        .read_line(&mut strng)
        .expect("something is wrong");
    s.push_str(&strng);
    println!("the new string formed is {}", &s);
    let new_arr = &s.as_bytes();
    for (i, &some) in new_arr.iter().enumerate() {
        println!("the i is {} and the some is {} ", i, some);
        if some == 32 {
            println!("text before spaces is {}", &s[..i]);
            break;
        }
    }
}

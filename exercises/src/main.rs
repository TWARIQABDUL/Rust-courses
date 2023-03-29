use std::io;
fn main() {
    println!("exercises 1 convert temperature ");
    println!("choose 1 to convert from fahrenheit_to_celsius and 2 from celsius_to_fahrenheit");
    let mut choice = String::new();
    let mut input = String::new();

    io::stdin().read_line(&mut choice).expect("something went wrong");
    println!("type in here");
    io::stdin().read_line(&mut input).expect("something went wrong");

    let choice:u32 = choice.trim().parse().expect("not a number");
    let input:f32 = input.trim().parse().expect("not a number");
    let fib = fib(1.0);
    if choice == 1 {
        fahr_temp(input);
    }else if choice == 2 {
        temperature_fah(input);
    }else {
        println!("invalid input🤢");
    }
    println!("the fib of {} is {}",10,fib);
}

fn temperature_fah(deg:f32){
    let fah = (deg * 1.8) + 32.0;
    println!("{} ℃  is {}℉", deg,fah);
}

fn fahr_temp(fah:f32){
    let cel = (fah - 32.0) * 0.5556;
    println!("{} ℉ is {} ℃", fah,cel);
    
}

fn fib(n:f32)->u32{
    if n == 0.0 {
         0
    }else if n == 1.0 {
        1
    }else {
        fib(n-1.0) + fib(n-2.0)
    }
}

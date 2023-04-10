#[derive(Debug)]
enum IpaddrClass {
    V4(u8,u8,u8,u8),
    V6(String)
}
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}
fn main(){
    let home = IpaddrClass::V4(192,168,1,23);
    println!("ipv4 {:#?} ",home);

    println!("the calculator");
    let op1= 23;
    let op2 = 21;
    let res = calculate(op1,op2,Operation::Add);
    println!("{} {} {} = {}",op1,operatin_sign(Operation::Add),op2,res);
}

fn calculate(operand1:u32,operand2:u32, operation: Operation)->u32{
    match operation {
        Operation::Add => operand1 + operand2,
        Operation::Subtract => operand1 - operand2,
        Operation::Multiply => operand1 * operand2,
        Operation::Divide => operand1 / operand2,
    }
}

fn operatin_sign(opera :Operation)-> String{
    match opera {
        Operation::Add => String::from('+'),
        Operation::Divide => String::from('/'),
        Operation::Subtract => String::from('-'),
        Operation::Multiply => String::from('*'),
    }
}
pub mod serving{
    use rand::Rng;
    use crate::back_of_house::chief;

    pub fn greet_customer(){
        let waiter = rand::thread_rng().gen_range(0..=2);
        let my_array: [String;3] = [String::from("Kalen"),String::from("Jacob"),String::from("stacy")];
        println!("Hello I'm {} what can I do for you",my_array[waiter]);
        menu();
    }
    pub fn get_order(order:&str){
        let plates: &str = order;
        // take order to the cheif
        chief::prepare_order(plates);
    }
    pub fn serve_order(){
        println!("Order served");
    }
    fn menu(){
        println!("1: Burger 200frw");
        println!("2: chicken 200frw");
        println!("3: flies 200frw");

    }
}
mod staff{
    pub fn reciept(item:&str){
        println!("you have ordered {} thanks for using our service",item);
    }
}
mod chief{
    use crate::back_of_house::serving;
    use crate::back_of_house::staff;
    pub fn prepare_order(ord:&str){
        take_order(ord);

    }
    fn take_order(ord:&str) {
        println!("order placed");
        staff::reciept(ord);
        serving::serve_order();


    }
}
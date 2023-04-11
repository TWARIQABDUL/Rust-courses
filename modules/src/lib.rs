mod front_house {
    pub struct Breakfast {
        pub toast: String,
        fruit: String,
    }
    impl Breakfast {
        pub fn summer(fr: &str) -> Breakfast {
            Breakfast {
                toast: String::from(fr),
                fruit: String::from("banana"),
            }
        }
    }
    pub mod hosting {
        pub fn add_to_host() {}
        fn seat_at_table() {}
    }

    pub mod serving {
        fn take_order() {}
        pub fn serve_order() {}
        fn take_payment() {}
    }
}

mod back_of_house {
    pub fn fix_order() {
        cook_order();
        super::front_house::serving::serve_order();
    }
    fn cook_order() {}
}

pub fn eat_at_restourant() {
    use front_house::Breakfast;
    crate::front_house::hosting::add_to_host();
    front_house::hosting::add_to_host();
    let mut meal =Breakfast::summer("beer");
    println!("{}",meal.toast);
}

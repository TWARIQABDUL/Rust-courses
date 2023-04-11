pub struct Breakfast{
    pub toast:String,
        desert_fruit:String
}
impl Breakfast{
    pub fn summer(tst: str) -> Breakfast{
        Breakfast{
            toast:String::from(tst),
            desert_fruit: String::from("caripso")

        }
    }
}
#[derive(Debug)]
pub struct User{
    pub uname:i32,
    pub active: bool,
    pub password:i32,
    pub login_time:i32
}
pub struct  Xxx;

impl User {
    pub fn hell()->User{
        User{
            uname:32,
            active:false,
            login_time:1,
            password:23
        }
    }
}
pub mod users{
    
    use std::collections::HashMap;
    pub fn create_user()-> HashMap<i32, &'static str> {
        let mut user = HashMap::new();
        user.extend(vec![(1,"Twariq"),(2,"sami"),(3,"jack Ass")]);
        user
    }
    fn login(use_name:String,pwd:String){
        create_user();
    }
}
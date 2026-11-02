use std::string;

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn main() {
    let user: User = User {
        username: String::from("sudheer"),
        email: String::from("samratsudheerdas@gmail.com"),
        active: true,
        sign_in_count:0
    };
    // accessing struct using dot operator
    println!("User details are username {}, email {}, {}, {}",user.username,user.email, user.active, user.sign_in_count);
    
    let mut user2: User = User {
        username: String::from("samrat"),
        email: String::from("samrat@gmail.com"),
        active: true,
        sign_in_count:5
    };
    user2.sign_in_count = 98;
    user2.active = false;
    println!("{},{},{},{}",user2.email,user2.username,user2.active,user2.sign_in_count);

    let user3 = build_user(String::from("shishir"), String::from("shishir@gmail.com"));
    println!("{}",user3.email);
}

fn build_user(username:String,email:String) -> User {
    User {
        username,
        email,
        active:true,
        sign_in_count:3
    }
}
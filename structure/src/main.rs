
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color (i32, i32, i32);

fn build_user(email:String, username:String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn print_user(user: &User) {
    println!("username:{}, email:{}, sign_in_count:{}, acitve:{}", user.username,
        user.email,
        user.sign_in_count,
        user.active);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let user_1 = User {
        username: String::from("sion"),
        email: String::from("sion@aaa.com"),
        sign_in_count: 1,
        active: true,
    };

    print_user(&user_1);

    print_user(&build_user(String::from("sion@bbb.com"), String::from("sion")));


    let user_2 = User {
        email: String::from("sion@ccc.com"),
        ..user_1
    };

    print_user(&user_2);

    let black = Color(0, 0, 0);
    println!("{} {} {}", black.0, black.1, black.2);


    let rectangle = Rectangle {
        width:30,
        height:40,
    };

    println!("{:?}", rectangle);
}

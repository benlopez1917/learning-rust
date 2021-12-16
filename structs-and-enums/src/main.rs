fn main() {
    // Structs
    let mut user1 = build_user(String::from("benlopez1917@tamu.edu"), String::from("benlopez1917"));

    user1.email = String::from("bendude7@gmail.com");

    let user2 = User {
        username: String::from("DrJimmy"),
        ..user1
    };

    let rect = Rectangle {
        width: 50,
        height: 100,
    };
    println!("Rectangle has area {}", rect.area());

    let square = Rectangle::square(4);

    //Enums

    let four = IpAddrKind::V4(String::from("127.0.0.1"));
    let six = IpAddrKind::V6(String::from("::1"));

    let addrtype = match six{
        IpAddrKind::V4(_) => String::from("V4"),
        IpAddrKind::V6(_) => String::from("V6"),
        _ => String::from("Unknown"),
    };
}

enum IpAddrKind {
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}
struct User {
    active: bool,
    username: String,
    email: String, 
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

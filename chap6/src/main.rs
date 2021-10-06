fn main() {
    // println!("Hello, world!");
    main2()
}

// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }

// fn create_user() {
//     let mut user1 = User {
//         email: String::from("test@example.com"),
//         username: String::from("username123"),
//         active: true,
//         sign_in_count: 1,
//     };
//
//     user1.email = String::from("totomail example.com");
// }
//
// fn build_user(email: String, username: String) -> User {
//     User {
//         email,
//         username,
//         active: true,
//         sign_in_count: 1,
//     }
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main2() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of the rectangle is {:#?} square pixels.", rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

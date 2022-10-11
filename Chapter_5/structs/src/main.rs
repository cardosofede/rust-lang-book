struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { 
            width: size, 
            height: size
        }
    }
}


fn main() {
    // Structs
    let mut user1 = User {
        email: String::from("dardonacci@gmail.com"),
        username: String::from("dardonacci"),
        active: true,
        sign_in_count: 1,
    };

    let name = user1.username;
    user1.username = String::from("fibonacci");

    // Using a function to create a new user
    let user2 = build_user(
        String::from("dardonacci@hotmail.com"), 
        String::from("Dardoneeeero"));

    // Using the information of other struct to complete the remaining fields
    let user3 = User{
        email: String::from("bobjames@gmail.com"),
        username: String::from("Bob James"),
        ..user2
    };

    // Tuple Struct
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    // Implement methods in structs
    let rect = Rectangle{
        width: 30,
        height: 45,
    };

    println!("Rectangle {:#?}", rect);

    println!(
        "The area of the rectangle is {} square pixels.", rect.area()
    );
    let rect1 = Rectangle {
        width: 20,
        height: 40,
    };

    let rect2 = Rectangle {
        width: 40,
        height: 50,
    };

    println!("Rect can hold rect1: {}", rect.can_hold(&rect1));
    println!("Rect can hold rect2: {}", rect.can_hold(&rect2));

    // Associated functions don't use self.
    let rect3 = Rectangle::square(20);
    println!("Rect can hold rect1: {}", rect.can_hold(&rect3));
    

}

fn build_user(email: String, username: String) -> User {
    User { 
        username,
        email, 
        sign_in_count: 1, 
        active: true
    }
}
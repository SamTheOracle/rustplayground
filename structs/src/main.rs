fn main() {
    println!("Hello, world!");
    let u = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        // active: u.active,
        // sign_in_count: u.sign_in_count,
        //we can also use:
        ..u
    };
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
    //if i don't use the pointer, i get a compile error because the rect1 has already been moved
    //println!("{}", area(rect1));
}
struct Rectangle {
    width: u32,
    height: u32,
}
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
//this is a tuple struct
struct Color(i32, i32, i32);
//rust associate email of the param to the user struct email
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

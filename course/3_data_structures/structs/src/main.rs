struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
struct Rectangle_WithFunction {
    width: u32,
    height: u32,
    // Storing a function as data
    calc_area: fn(u32, u32) -> u32, 
}

struct Square {
    width: u32,
}
fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // this is called struct update syntax, it copies the remaining fields from user1
    };
    println!("user2 username is: {}", user2.username);

    let user3 = build_user(String::from("help@example.com"), String::from("helpuser123"));

    let white = Color(255, 255, 255);
    let origin = Point(0, 0, 0);

    println!("white color is: ({}, {}, {})", white.0, white.1, white.2);
    let subject = AlwaysEqual;

    let subject2 = AlwaysEqual;

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {} square pixels.", rect1.area());

    let rect2 = Rectangle_WithFunction {
        width: 30,
        height: 50,
        calc_area: |width, height| width * height, // using a closure to calculate the area
    };
    println!("The area of the rectangle is {} square pixels.", (rect2.calc_area)(rect2.width, rect2.height));

}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

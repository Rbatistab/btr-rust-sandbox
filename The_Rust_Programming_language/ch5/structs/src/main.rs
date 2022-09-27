#[derive(Debug)]
struct User {
    username:           String,
    email:              String,
    sign_in_account:    u64,
    active:             bool,
}

#[derive(Debug)]
struct Rectangle {
    width:  u32,
    height: u32,
}

// Methods are similar to functions, however the are defined within the concept of a struct
// First parameter should always be 'self'
impl Rectangle {
    fn area(&self) -> u32 {
        // Methods can take ownership of self or borrow it
        self.width * self.height
    }

    fn set_witdth(&mut self, width: u32) {
        self.width = width;
    }

    // Asociated functions don't have an instance hence don't need self
    // Often used for constructors that will return a new instance of the struct
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }

    fn to_string(&self) -> String {
        let mut string_form = String::from("");
        string_form.push_str(&self.width.to_string());
        string_form.push_str("x");
        string_form.push_str(&self.height.to_string());
        string_form
    }
}

fn main() {
    // Struct: Custom data type to package together multiple values
    // "The object's attributes"
    // Similar to tuples but allows different types and naming the fields
    let mut user1 = User { // The whole instance has to be mut, not only certain values
        email:              String::from("1@2"),
        username:           String::from("user 1"),
        active:             true,
        sign_in_account:    1,
    };

    user1.username = String::from("Alex");
    println!("{:?}", user1);
    println!("");

    let user2 = build_user(
        String::from("email@email"), 
        String::from("John")
    );
    println!("{:?}", user2);
    println!("");

    // Tuple structs look similar to touples

    #[derive(Debug)]
    struct Color(i32, i32, i32);
    #[derive(Debug)]
    struct Point(i32, i32, i32);

    let black = Color(0,0,0);
    let origin = Point(0,0,0);

    println!("{:?}", black);
    println!("{:?}", origin);
    println!("");


    let mut rect1: Rectangle = Rectangle { width: 30, height: 15 };
    let area: u32 = area(&rect1);
    println!("The area of {:?} is {}", rect1, area);
    
    rect1.set_witdth(20);
    let area2 = rect1.area();
    println!("The area of {:?} is {}", rect1, area2);
    println!("");

    let a_square = Rectangle::square(25);
    println!("The area of {} is {}", a_square.to_string(), a_square.area());
    println!("");



}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn build_user(email: String, username: String) -> User {
    User { 
        username, 
        email, 
        sign_in_account:    1, 
        active:             true 
    }
}

// Collections are stored in the heap, hence doesn't need to be known at compile time
//  - Vector -> Variable number of, same type, values next to each other
//  - String -> Colleciton of characters
//  - HasMap -> Key-value (particular implementation of map)

enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    println!("Hello, collections!");

    // Vectors are implemented using generics
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    // println!("{}", v);
    // It's often more common to create Vec<T> with initial values and use the `vec!` macro
    let mut v2 = vec![1,2,3];
    v2.push(4);
    // println!("{}", v2);
    
    // The only 2 ways to get a value from a vector are:
    //  1) & with []    -> reference (may panic)
    //  2) get()        -> Option<T> (handles a panic with None)
    let third: &i32 = &v[2];
    println!("{}", third);
    println!("");
    
    match v.get(3) {
        Some(fourth)    => println!("{}", fourth),
        None            => println!("No fourth element!"),
    }
    println!("");
    
    // When there's a valid reference, the borrow checker enforcers ownership and borrowing rules:
    let first_v = &v[0]; // inmutable borrow
    // v.push(8);
    // I wont be able to do this mutable borrow for v.push, since I have an inmutable borrow with first_v
    println!("The first is {}", first_v);
    println!("");
    // Adding a new element to the end might require memory allocation and copying the vector to a new space, if there isn't enoug room
    // to put all the elements next to each other, hence the reference to the first would point to deallocated memory, rules won't allow it
    v.push(8); // Now that I let go of first_v, I can push to v again
    println!("");
    
    // Inmutable Iteration:
    let v3: Vec<i32> = vec![100,45,63];
    for number in &v3 {
        println!("{}", number);
    }
    println!("");
    
    // Mutable Iteration:
    let mut v4: Vec<i32> = vec![100,45,63];
    for number in &mut v4 {
        // Remember to use the dereference operator
        *number *= 3;
        println!("{}", number);
    }
    println!("");
    
    // Enums: 
    // Vectors can't hold any type, since there's a chance that any of the types could cause errors
    // The safest path to reproduce such behaviour is with an enum and a match
    // *** this can only work is you know an exhaustive set of types before runtime, else won't work
    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(1.2),
        SpreadSheetCell::Text(String::from("Sapbe")),
    ];
    for cell in &row {
        match cell {
            SpreadSheetCell::Int(my_int) => println!("{}", my_int),
            SpreadSheetCell::Float(my_float) => println!("{}", my_float),
            SpreadSheetCell::Text(my_text) => println!("{}", my_text),
        }
    }
    println!("");

    // Strings are a collection of bytes plus some methods
    // The only string type is the slice string str, which are references to some UTF-8 encoded string data stored somewhere else
    // The String type is provided by std lib and is a growable, mutable, ownerd UTF-9 encoded string type
    // Many of the methods available with Vec are with String
    let mut s: String   = String::new();
    let data: &str      = "Initial content";
    let s2: String      = data.to_string();
    s.push_str(data);
    println!("{}", s);
    println!("{}", s2);
    println!("");
    
    // Concatenation:
    let mut s3: String = String::from("foo ");
    let s4: &str = "bar ";
    // push_str() takes a slice since it doesn't want to take ownershi
    s3.push_str(s4);
    // push() pushes only one char
    s3.push('c');
    s3.push('h');
    s3.push('u');
    println!("{}", s3);
    println!("{}", s4);
    println!("");
    
    // The `+` operator (generic add method), we can only add str slice to an string
    // Rust can coerce the &String into a &str by using a deref coercion, turning s6 into &s6[..] without taking ownership
    let s5: String = String::from("foo");
    let s6: String = String::from("bar");
    // Here we'll lose s5 since it's moved to s7
    let s7: String = s5 + " " + &s6;
    println!("{}", s7);
    println!("");
    
    // The format!() macro returns a String in a println!() like manner, without taking ownership
    let s8: String = String::from("Hello");
    let s9: String = format!("{} world!", s8);
    println!("{}", s9);
    println!("");

    // Rust strings don't support indexing:
    // let s10: String = String::from("Hello!");
    // let h = s10[0];

    // Internal representation:
    // A string is a wrapper over a Vec<u8>, one byte per character
    // If you have unicode, remember each Unicode scalar takes 2 bytes, hence an index is not a good representation for a character
    // There will be 3 ways to look at strings:
    //  - Bytes:
    //  - Scalars:
    //  - Grapheme clusters(~ letters):
    // let hello: String = String::from("привет");
    // let answer = &hello[0];

    println!("");
    println!("");
    println!("");
}


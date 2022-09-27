fn main() {
    // Ownership is the central feature for RUST, as a set of rules the compiler checks at runtime
    // In other languagues you must explicity dellocate memory
    // Data in the stack is faster than the heap
    // Ownership Rules:
    //  1) Each value has a variable called "owner"
    //  2) Only one owner at the time
    //  3) Value is droped once the owner is out of scope
    println!("Ownership!\n");
    let my_str: &str = "hello, I'm inmutable, known in compile time and in the stack";
    // Rust has str (sting literals) and Strings
    //  str     -> stack, they are inmutable
    //  String  -> heap, amount of text is unknown at compile time
    let mut my_sring: String = String::from("hello");
    // Here the "::" operator allows us to namespace this particular function (from) under String
    // "Garbage collection":
    //  1) String::from requests the memory it needs
    //  2) memory is returned once the owner goes out of scope by calling 'drop' function
    my_sring.push_str(" world! I live in the heap");

    println!("{}", my_str);
    println!("{}", my_sring);

    println!("");
    
    // Integers can be copied, their size is known at compile time and go to the stack and are quick to make
    // Here is no diference between deep and shallow copies, a clone makes no difference as well
    let x: i32 = 5;
    let y: i32 = x;
    println!("{}", x);
    println!("{}", y);

    println!("");

    // Integers can be copied, their size is known at compile time and go to the stack
    let s1: String = String::from("Hello");
    let s2: String = s1;
    // println!("{}", s1); // This value was borrowed to s2, s1 can't be used
    println!("{}", s2);
    // A string is made of (the group is stored in the stack):
    //  1) Pointer to memory
    //  2) Lenght (memory in bytes)
    //  3) Capacity (total amount of memory in bytes that it received from the OS)
    // When making s1 = s2 we copy String data (above) but not the heap data where the characters live
    // When both s1 and s2 go out of scope they will try to free same memory => double free error
    // so this is why rust considers s1 as no longer valid, hence it doesn't free anything after s1 is out of scope
    // Move: invalidating s1, s1 is moved to s2
    // Rust will never allow deep copies
    println!("");

    let s3: String = String::from("hello");
    // This is clone but is expensive, since it clones the heap
    let s4: String = s3.clone();
    println!("{}", s3);
    println!("{}", s4);

    println!("");

    // Copy trait (special annotation)
    // When a type has a Copy trait the older variable is still usable after assignment
    // Rust won't let annotate a type with copy if the type has a Drop trait
    // Copy ex: ints, bools, chars, floats or tuples made of those

    // Fns:
    // The ownership with functions works the same

    let s5: String = String::from("hello");
    let x: i32 = 5;

    takes_ownership(s5);
    makes_copy(x);
    // println!("{}", s5); // this was moved
    println!("{}", x);
    
    let s6: String = String::from("hello");
    let s7: String = takes_and_gives(s6);
    println!("{}", s7);

    println!("");

    // Always assigning a heap-like variable will transfer the ownership


    // References:
    // You can use a reference instead of taking the ownership
    let s8: String = String::from("hey!");
    let len: usize = calculate_len(&s8);
    // Len rerefers to value of s8 but doesn't own it
    println!("{}", s8);
    println!("{}", len);
    // The oposite of reference(&) is dereference(*)

    // Borrowing: Call having references ;)
    // References are inmutable by default
    let mut s9: String = String::from("hello");
    let mut s10: String = String::from("hello");
    // mutate_a_string(&s9); // this refrence won't mute
    mutate_a_string(&mut s10);
    println!("{}", s9);
    println!("{}", s10);

    // Rust prevents data races at compile time
    //  - 2 or more pointes access same data at the same time
    //  - At least one pointer writes data
    //  - There's no mechanism to synchronize access data
    let mut s11: String = String::from("hello");
    let r1: &mut String = &mut s11;
    // let r2 = &mut s11;

    // print!("{}, {}", r1, r2);
    println!("{}", r1);

    println!("");
    // However a block can remove it from the scope
    let mut s12: String = String::from("hello");
    {
        let r3: &mut String = &mut s12;
        println!("{}", r3);
    }
    let r4: &mut String = &mut s12;

    println!("{}", r4);

    println!("");

    // Multiple inmutable references are OK, but can't be mixed with mutables
    // neither more than one mutable
    // Rules of reference:
    //  1) At any moment you can have eiher but not botho of: one mut or any inmut
    //  2) References must always be valid


    // Slice type:
    // Does not have ownership, allows to reference a contiguos space of elements, rather than the whole
    let mut s13: String = String::from("Hello slice!");
    let word: usize = first_word(&s13);
    s13.clear();
    println!("{}", word);
    
    let s14: String = String::from("Hello slice!");
    // String slices are a reference to a portion of the string
    // They must occur at valid UTF-8 character boundaries, attempting to do it in the middle of a 
    // multibyte character will exit on error
    let s14_1: &str = &s14[0..5];
    println!("{}", s14_1);
    let s14_2: &str = first_word_str(&s14);
    println!("{}", s14_2);
    // s14.clear(); // This fails since s14_2 borrowed a slice as inmutable

    println!("");

}

fn first_word_str(s: &String) -> &str {
    let bytes: &[u8] = s.as_bytes();
    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..index];
        }
    }

    return &s;
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    // iter() returns every element in a collection
    // enumerate() wraps the result of iter and returns each element as a part of a tuple (index, reference to element)
    for (i, &item) in bytes.iter().enumerate() {
        // If we find a space we return the current index
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn mutate_a_string(some_string: &mut String){
    some_string.push_str(" world!");
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn takes_and_gives(some_string: String) -> String {
    some_string
}

fn calculate_len(s: &String) -> usize {
    s.len()
}
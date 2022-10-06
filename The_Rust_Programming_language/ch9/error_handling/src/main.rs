// There are 2 types of errors:
//  > Unrecoverable(panic!)     -> Sympotom of bugs
//      > Nothing to do about them will print a failure message:
//      > Unwinds and cleans the stack and quit
//      > Alternative is to abort which ends the program without cleaning (OS must clean instead) 
//      > Run 'RUST_BACKTRACE=1 cargo run' to have a printed list of the error trace
//    
//  > Recoverable(Result<T,E)   -> Report and retry
//      > Remember 'Result' Type -> 'enum Result<T, E> { Ok(T), Err(E), }', Err is a generic Error variant
//      > 'Result' is brought into scope by the prelude, needless to add 'Result::Ok' or 'Result::Err' 

// Common languages don't distinguish between both and handle with exceptions\

use std::{io::{ self, ErrorKind, Read }, fs::File};


fn main() {
    println!("Hello, errors!");
    make_me_panic();
    make_me_handle_error();
    propagate_error();
}

fn make_me_panic() {
    // panic!("Crashing, unwindind and quiting");
    // Rust won't let you access any memory, it won't bufer overread, this is a security vulnerability
    // let v: Vec<i32> = vec![1,2,3];
    // v[99];
}

fn propagate_error() {
    // Propagating allows to return the error to the calling function to be handled
    read_from_file_long_way();
    read_from_file_shorter_way();
    read_from_file_even_shorter();
}

fn read_from_file_long_way() -> Result<String, io::Error> {
    use io::{ self, Read };
    use std::fs::File;

    let f = File::open("Hello.txt");

    let mut f = match f {
        Ok(file)    => file,
        Err(error)  => return Err(error),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_)   => Ok(s),
        Err(e)  => Err(e), // No need of return keyword here, this is the last expression
    }
    // We don't know what the calling code will do with these valies
}

// The ? Opetator:
//  > will call `from` function is used to convert errors from one type to another
//  > Only works for functions with a return type Result (won't work in main unless it's output is modified)
//  > Useful in a function that one error type that represents all the ways it might fail
fn read_from_file_shorter_way() -> Result<String, io::Error>{
    let mut f   = File::open("Non_existing_file")?;
    let mut s       = String::new();
    f.read_to_string(&mut s)?;
    
    // Will only return this Ok if there was an Ok, if not an Err will be returned instead
    Ok(s)
    
    // ? will eliminate a lot of boiler plate


    // One way to make it shorter:
    // let mut s = String::new();
    // File::open("some_file")?.read_to_string(&mut s)?;
    // Ok(s)
}

fn read_from_file_even_shorter() -> Result<String, io::Error> {
    use std::fs;
    fs::read_to_string("Dont_exist")
}

fn make_me_handle_error() {
    use std::fs::File;
    use std::io::Error;

    // F uses an Error from io, which is why it is into scope
    let f: Result<File, Error> = File::open("I_dont_exist");

    let f = match f {
        Ok(file)    => file,     // Instance of Ok
        Err(error)  => {         // Instance of Error, in this case a variant of io::Error
            // You could Panic if you need:
            // panic!("Problem! Error: {:?}", error)

            // Or handle the Error type:
            match error.kind() {
                // Instance of io::ErrorKind that represents diferent errors from an io operation
                ErrorKind::NotFound => match File::create("I_do_exist") {
                    Ok(fc)  => fc,
                    Err(e)  => panic!("Can't create file: {:?}", e),
                },
                other_error => panic!("Can't open the file: {:?}", other_error),
            }
        },
    };
    // Match will handle well but could be a bit too much verbose as to communicate something well

    // A more rustacean way is the unwrap helper:
    let f = File::open("Still_don't_exist").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("Now_I_exist").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // A similar method is expect:
    // expect may be a bit easier than unwrap since the message could allow it to be more searchable than
    // having to look among too many unwraps
    let f = File::open("Again_I_dont_exist").expect("Failed to open the file");

}

// *** Panic vs not panic: There are some times when it's more appropiate to panic rather than handling
// 
//  * Yes, panic! if:
//      > Wrinting an example (don't clutter intention with very large code)
//      > Prototyping (use wrap/unwrap before deciding how to handle)
//      > Testing, panic is the best test to show a failure
//      > Could end in a bad state
//      > External APIs return an invalid state
//      > You make external APIs enter an invalid state (bad use of your code)
//      > When an API/function contract is violated --> Caller side bug
// 
//  * Not necessary panic if:
//      > You can ensure you won't have en Err variant (no need to overkill)
//      > The bad state happens ocassionally
//      > Some code needs to rely on a bad state
//      > No good way to encode error information
//      > Failure is expected
//      > Rust type system can verify you have something valid (don't overkill, again)


// Input validation example:
struct PositiveNumberLesserThanOneHundred {
    value: i32,
}

impl PositiveNumberLesserThanOneHundred {
    fn new(value: i32) -> PositiveNumberLesserThanOneHundred {
        if value < 1 || value > 100 {
            panic!("Number should be between 1 and 100, got {}.", value);
        }

        PositiveNumberLesserThanOneHundred {
            value
        }
    }

    fn value(&self) -> i32 {
        self.value
    }
}
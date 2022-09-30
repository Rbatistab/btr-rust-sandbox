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

use std::io::{ self, ErrorKind };


fn main() {
    println!("Hello, errors!");
    make_me_panic();
    make_me_handle_error();
    propagate_error();
}

fn propagate_error() {
    // Propagating allows to return the error to the calling function to be handled
    read_from_file_long_way();
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

fn make_me_panic() {
    // panic!("Crashing, unwindind and quiting");
    // Rust won't let you access any memory, it won't bufer overread, this is a security vulnerability
    // let v: Vec<i32> = vec![1,2,3];
    // v[99];
}

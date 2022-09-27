use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {

    let secret_fucking_number = rand::thread_rng().gen_range(1..=100);

    println!("Ha! the secret fucking number is {}", secret_fucking_number);

    loop {
        
        println!("Guess a number: ");
        
        // Rust variables are inmutable by default
        // String::new ->   associated function new, of String type,
        //                  they are implemented on a type rather than
        //                  on an instance
        // associated fn ~ static method
        let mut guess = String::new();
        
        // & -> Reference those are inmutable by default as well
        //      if it were only &guess it wouldn't work
        // read_line -> expexted to return an io::Result
        //              Result is a Rust enum variant
        io::stdin().read_line(&mut guess)
            .expect("You messed it up! User fault");
        
        // Rust allows to "shadow" previous value with a new one
        // this let us reuse rather than create 2 unique values
        // trim     -> remove white space (\n)
        // parse    -> string to number
        // let guess: u32 = guess.trim().parse()
            // .expect("Type a god damn number! Fuck!");
        
        // This is how you rather handle from crashing to handling errors
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue,
        };
          
        
        println!("You guessed: {}", guess);
        
        // A match expression is made up of "arms"
        // These arms are the variants of Ordering (another enum)
        match guess.cmp(&secret_fucking_number) {
            Ordering::Less      => println!("Tiny wini"),
            Ordering::Greater   => println!("It's so big"),
            Ordering::Equal     => {
                println!("Just fit perfect, IYKWIM");
                break;
            }
        
        }
    }
}

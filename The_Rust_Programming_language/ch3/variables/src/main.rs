use core::num;
use std::io;

fn main() {
    // Functions in rust are by snake_case
    // and it doesn't matter where they are, just that they are
    // FN bodies are a series of statements optionally ending in an expression

    // RUST is an expression-based language
    // Statements perform some action without returning
    // Expressions evaluate to return a value (calling an fn or macro is an expression)
    variables_constants_and_mutability();
    data_types();
    data_types_index_out_of_range();
    function_with_params(1, 1.1);
    expressions_and_statements();
    println!(
        "A function with a return is and expression that returns: {}\n", 
        a_return_fn()
    );

    if_else_conditons();
    looping();

}

fn looping() {
    let mut counter = 1;

    let result_loop = loop {
        counter += 1;
        if counter == 10 {
            break counter;
        }
    }; // I am an statement, inside the block there is an expression
    println!("result_loop: {}", result_loop);
    
    while counter > 5 {
        counter -= 1;
    }
    println!("counter: {}", counter);

    let my_array = [1,2,3,4,5,6];

    for number in my_array.iter() {
        println!("my array_current = {}", number);
    }
    
    for number in (1..5).rev(){
        println!("in other for(ruby like) = {}", number);
    }

    println!("");
}

fn if_else_conditons() {
    // if conditions only take booleans, there is no implicit conversion like "if number" like in ruby
    let x = 1;
    if x > 5 { // I am an "arm"
        println!("true");
    } else {
        println!("false");
    }

    let y = 3;
    if y == 1 {
        println!("one");
    } else if y == 2 {
        println!("two");
    } else if y == 3 {
        println!("three");
    }

    let is_true = true;
    let x = if is_true {
        1
    } else { 
        // They should be the same type at compile time, not at run time
        //"2" -> this would fail to compile
        2
    };
    println!("The value of x is {}", x);
    println!("");

}

fn a_return_fn() -> u8 {
    1 // I am an expression, I don't have a semicolon
}

fn expressions_and_statements() {
    let num = 5;

    let an_statement = {
        // This inside block is an expression
        let this_expression = 3;
        this_expression + 3 // expressions do not end with semicolon, a semicolon turns into ans statement
    }; 
    println!("This statement comes from an expression: {}", an_statement);
    println!(""); // I end in semicolon because this particular function is an statement rather than an expression

}

// Signatures MUST include data type
fn function_with_params(x: u8, y: f32) {
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
    println!("");
}

fn data_types_index_out_of_range() {
    let my_aray = [1,2,3,4,5];
    println!("Input an index:");

    let mut index = String::new();
    io::stdin().read_line(&mut index)
        .expect("failed to read line");
    let index: usize = index.trim()
        .parse()
        .expect("Not a number");

    let element = my_aray[index];
    println!(
        "The value at {} is {}", 
        index, element
    );
    println!("");
}

fn data_types() {
    // Every value is typed in RUST (statically typed), it must know type in compile time
    // Subsets: 
    //      * scalar    -> integers, floating poinrs, booleans, characters
    //      * compound  -> multiple values, primitives are tuples and arrays

    // ------- Scalars ---------

    let x_integer   = 4;
    let x_float         = 4.0;
    let x_boolean       = true;
    let x_char          = 'c';  // char literals are specified to single quotes('), strings are double qoutes("), also are unicode scalars
    println!("x_integer is {}", x_integer);
    println!("x_float is {}", x_float);
    println!("x_boolean is {}", x_boolean);
    println!("x_char is {}", x_char);
    println!("");


    // Rust allows basic math operations:
    let my_sum  = 1 + 1;
    let my_rest = 1 - 1;
    let my_mult     = 1*2;
    let my_div      = 4/2;
    let my_rem     = 9%4;
    println!("my_sum is {}", my_sum);
    println!("my_rest is {}", my_rest);
    println!("my_mult is {}", my_mult);
    println!("my_div is {}", my_div);
    println!("my_rem is {}", my_rem);
    println!("");
    
    // ------- Compounds ---------
    // Tuples group some number of variables
    let my_tuple    = (500, 6.4, 'c');
    let (a,b,c) = my_tuple; //This is called "destructing" the touple
    println!("a is {}", a);
    let other_tuple: (i32, f64, u8) = (20,2.8,1);
    let one = other_tuple.2;
    println!("one is {}", one);
    println!("");
    
    // ------- Arrays ---------
    // Fixed length, all values of the same type
    // Useful when data is in the stack rather than the heap
    // Not as flexible as vector, is unsure, then use vector
    let my_array = [1,2,3,4,5,6];
    let days_better_as_array_than_vector  = ["Monday", "Tuesday", "etc"];
    let arr_with_type: [u16; 5] = [1,2,3,4,5];
    let five_threes = [3;5];
    println!("a three is {}", five_threes[0]);
    println!("");

}

fn variables_constants_and_mutability() {
// For large data structures is better to mutate.
    // For smaller is better to create a new instance in a more functional style is the easier way to think
    let mut x = 5;
    println!("X is {}", x);
    x = 6;
    println!("X is {}", x);
    println!("");

    // Constant must be set to a constant expression, not to a call to function or any value at run time
    const PI: f32 = 3.14159266;
    println!("PI is {}", PI);
    println!("");


    // Shadowing:
    // First 'y' is shadowed by second 'y'
    // 'y' shadows 'y' by repeating the let
    // 
    // This allows to "mute" the value of y without loosing inmutability
    let y = 3;
    let y = y + 5;
    let y = y*2;
    println!("Y is {}", y);
    let y = "hello!";
    println!("Y is {}", y);
    println!("");
    
    // This wouldn't work since the 'let' is used indise a loop block, it won't "mute" y
    // let y = 0;
    // loop {
    //     if y == 10 {
    //         break
    //     }
    //     let y = y + 1;
    // }
    // println!("Y is {}", y);
}

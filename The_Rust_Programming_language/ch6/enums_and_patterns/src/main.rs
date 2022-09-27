use core::fmt;
enum IpAddrkind {
    V4(u8, u8, u8, u8),
    V6(String),
}
impl fmt::Display for IpAddrkind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IpAddrkind::V4(x,y,z,w) => write!(f, "V4"),
            IpAddrkind::V6(addr) => write!(f, "V6"),
        }
    }
}

struct IpAddr {
    kind:       IpAddrkind,
    address:    String,
}

impl IpAddr {
    fn to_string(&self) -> String {
        let mut output_string: String = String::new();
        output_string.push_str(&self.kind.to_string());
        output_string.push_str(" / ");
        output_string.push_str(&self.address);
        output_string
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alaska,
    Alabama,
}

fn main() {
    // Enums allow to define a type by enumerating possible values
    let four: IpAddrkind    = IpAddrkind::V4(127,0,0,1);
    let six: IpAddrkind     = IpAddrkind::V6(String::from("::1"));

    let home:       IpAddr = IpAddr {
        kind:       four,
        address:    String::from("127.0.0.1")
    };
    let loopback:   IpAddr = IpAddr { 
        kind:       six, 
        address:    String::from("::1"),  
    };

    println!("Home: {}", home.to_string());
    println!("loopback: {}", loopback.to_string());
    println!("");

    // The option Enum encodes a value that could be something or could be nothing
    // compiler handles according to the case, there is no 'null' feature in Rust
    // Trying to use null in a no-null value raises and error
    // A null value is invalid or absent:
    // enum Option<T> {   -> this is implicit, don't write it
    //     Some(T),
    //     None,
    // }
    
    let some_number: Option<i32>    = Some(5);
    let some_string: Option<&str>   = Some("str");
    let absent_number: Option<i32>  = None;

    // let the_number:i32 = match some_number {
    //     Some(x: i32)  => x,
    //     // None            => "Non" 
    // };

    // you need to convert from Option<T> to T before using it
    // Also you need to handle the case for when the value is null
    // println!("{}, {}", some_number, some_string);
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // The '_' placeholder ("default")
    let some_value = 0u8;
    match some_value {
        1 => println!("One!"),
        2 => println!("Two!"),
        3 => println!("Three!"),
        _ => println!("Nah!"),
    }

    let some_u8 = Some(0u8);

    // An if let instruction is short and elegan:
    if let Some(3) = some_u8 {
        println!("Three!");
    }
    println!("");
    println!("");
    
    println!("");
    println!("");
}

// fn route(ip_kind: IpAddrkind) { }

fn value_in_cents(coin: Coin) -> u8 {
    // An if returns a boolean, a match return anything you need it to return
    match coin {
        // Pattern => some code
        Coin::Penny     => {
            println!("lucky goddamn penny!");
            1
        },
        Coin::Nickel    => 5,
        Coin::Dime      => 10,
        Coin::Quarter(UsState) => {
            println!("A quarter from {:?}", UsState);
            25
        },
        
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // Combining match and Some is very common in RUST
    match x {
        // RUST is exhaustive and it will complaint if we don't cover all matched values
        None => None,
        Some(i) => Some(i + 1),
    }
}
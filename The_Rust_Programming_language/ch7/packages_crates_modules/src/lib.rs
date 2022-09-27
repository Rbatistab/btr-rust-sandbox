pub fn hello_lib() {}


mod front_of_house {
  // Children encapsulate and hide form parents and making a  parent public doesn't modify the child
  pub mod hosting {
    pub fn add_to_waitlist() {}

    pub fn seat_at_table() {}
  }

  pub mod serving {
    pub fn take_order() {}

    fn serve_order() {}

    fn take_payment() {}
  }
}



mod back_of_house {
  pub struct Breakfast {
    // making an struct public doesn't make the fields inside public as well
    pub toast:      String,
    seasonal_fruit: String,
  }

  impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
      Breakfast { 
        toast:          String::from(toast), 
        seasonal_fruit: String::from("peaches"),
      }
    }
  }

  // Making an enum public (in contrast to struct) does make all of the fields public
  // Having private fields for an enum is not very usefull tho
  pub enum Appetizer {
    Soup,
    Salad,
  }

  fn fix_incorrect_order() {
    cook_order();
    // '..syntax' -> `../` analog to `super`
    super::front_of_house::serving::take_order();
  }

  fn cook_order() {}
}

// Now let's make life easier with the use word to avoid writing the whole path (absolute or relative):
// Use is similar to a symbolic link
use crate::front_of_house::hosting;
// We are not going all the way to `crate...::hosting::add_to_waitlist` since we wan an IDIOMATIC way
// We want the parent's module in to scope since we don't want that the function is being called locally
// We want to make clear that we want this function from this module, not from the current block (if any function with equal name)

// For structs, enums and others we want idiomatic to the full path (this one is by convention):
use std::collections::HashMap;

// Rust won't allow to bring 2 items with the same name from different modules with use statements:
//  use Hello1::my_item;
//  use Hello2::my_item;

// Take this example:
// use std::fmt;
// use std::io;
// fn my_function1() -> fmt::Result { 
//   //snip
// }
// fn my_function2() -> io::Result {
//   // snip
// }

// Now the `as` word is what you want you lazy buoy:
//  use std::fmt::Result as FmtResul;
//  use std::io::Result as IoResult;

// Re-exporting:
// The `use` keyword is also private by deafault, so the name is private as well
// To enable code to call our code use `pub`
pub use crate::front_of_house::serving; // I am public as fuck, and external code can call me
// re-exporting is usefull when the internal structure of your code is different from how programmers 
// calling your code would think about the domain
// You write your code with one structure but expose a different structure, this makes out library
// well organized for programmers working on the libreary and programmers calling the library

// Nesting use paths is the shit!:
use std::{ 
  io            as  IO, 
  cmp::Ordering as  MyOrd 
};

// Also you can do this:
//  use std::io;
//  use std::io::Write;
// As:
//  use std::{
//    self,
//    Write
//  };

// Glob Operator:
use std::collections::*; // I bring all public items from here

// Bringin something from another file:
mod side_of_house;  // The `;` tells rust to load the contents from another FILE WITH THE SAME NAME OF THE MODULE
pub fn the_side() {
  side_of_house::side_hosting::host_to_side();
}

// External folders: 
//  * https://doc.rust-lang.org/reference/items/modules.html#module-source-filenames
//  * https://hackernoon.com/including-files-and-deeply-directories-in-rust-q35o3yer
// "Rust doesn't see files as files, but it sees them as modules and files inside folders as sub-modules." 
//  Jose Javi Asilis. Including Files and Deeply Directories in Rust. 
mod more_house_mods;
use crate::more_house_mods::roof_of_house;
// #[path = "../out_folder/I_outside_of_src.rs"]
// mod out_folder::I_outside_of_src::outside;

pub fn explore_house() {
  more_house_mods::basement_of_house::dungeon::to_the_dungeons();
  roof_of_house::roof::roofing();
}


pub fn eat_at_restaurant() {
  // Absolute path:
  crate::front_of_house::hosting::add_to_waitlist();

  // Relative path:
  front_of_house::hosting::add_to_waitlist();

  // Struct interaction:
  let mut meal: back_of_house::Breakfast  = back_of_house::Breakfast::summer("Rye");
  meal.toast          = String::from("Wheat");
  // You won't be able to access not public attributes of the strcut
  // meal.seasonal_fruit = String::from("apple");
  print!("I'd like {} toast", meal.toast);

  // Enum interaction
  let order1: back_of_house::Appetizer  = back_of_house::Appetizer::Soup;
  let order2: back_of_house::Appetizer  = back_of_house::Appetizer::Salad;

  // Use of `use` keyword for `crate::front_of_house::hosting;`
  hosting::add_to_waitlist();

  // Idiomatic hasmap:
  let mut map: HashMap<u8, u8>  = HashMap::new();
  map.insert(1, 1);

}


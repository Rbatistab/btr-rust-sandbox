// Split in multiple modules, then multiple files
// A package can have multiple binary crates and optionally one library crate
// As package grows you can extract parts into separate crates that become external dependencies (IMPORTANT!!)
// Scope: nested context in which the code is written has a set of names defined as "in scope"
// Compilers need to know whether a particular name is variable, function, strcut, etc
// Module System:
//  1) Packages     ->  Cargo feature that allows build, test, and share crates
//                      Contains one or more crates that provite a set of functionality and zero or one lib crates (no more)
//                      Contains at least one binary crate
//                      It's "Cargo.toml" describes how to build the crates
//                      Multiple binary crates can be placed in the src/bin directory
//  2) Crates       ->  Tree of modules that produces a lib or exec (group functionality)
//                      "Crate root" -> source file that Rust compiler starts from and makes up the root of your crate
//                      src/main.rs is the crate root of a binary crate with the same name as the package
//                      Also if there's a src/lib.rs then it has a crate with the same name as the package (root)
//                      To place multiple binaries use src/bin, each file with a separate binary crate
//                      Crates group functionality
//                      rustc builds the binaries into bin
//  3) Modules/use  ->  Controls organization, scope and privacy of paths
//                      Organize code within a crate into groups for readability and reuse
//                      Control privacy (Rust makes it private by deafault)
//                      Can hold definitions (structs, enums, etc.)
//                      There's a module tree ex:
//                          crate
//                              front_of_house
//                                  hosting
//                                      add_to_waitlist
//                                      seat_at_table
//                                  serving
//                                      take_order
//                                      serve_order
//                                      take_payment
//  4) Paths        ->  Naming of an item(struct, func, etc)
//                      To show Rust where to find an item
//                      - Absolute  ->  From root crate(name or literal crate)
//                      - Relative  ->  From current module (self, super, etc)

fn main() {
    println!("Hello, world!");
    // hello_lib();
}

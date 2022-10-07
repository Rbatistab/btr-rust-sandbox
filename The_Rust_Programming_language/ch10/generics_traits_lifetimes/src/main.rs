// 1) Extract a function to reduce DRY
// 2) From the extracted function make a generic that differ only in the types
// 3) Use traits to define behavior in a genericway and combine traits to 
//     constraint a generic type to only those types with a particular behavior
// 4) Lifetimes, a variety of generics tell the compiler information about how the references relate to each other

fn main() {
    println!("Hello, generics, traits and lifetimes!");
    println!("");

    // 1) Reduce dry by extracting a function
    let list1: Vec<i32> = vec![10, 24, 24, 6687, 87];
    println!("The lesser in list1 is: {}", lesser(&list1));
    println!("");
    let list2: Vec<i32> = vec![354, 6, 23, 4, 45];
    println!("The lesser in list2 is: {}", lesser(&list2));
    println!("");
}

fn lesser(list: &[i32]) -> i32 {
    let mut lesser  = list[0];

    for &item in list.iter() {
        if item < lesser {
            lesser = item;
        }
    }

    lesser
}

// We have to declare the generic parameter in angle brackets `<>`
// Any name could work, but the standart is T
// fn generic_lesser<T>(list: &[T]) -> T {
//     let mut lesser: T   = list[0];

//     for &item in list.iter() {
//         // This will pop an error: We can only use types whose values can be ordered
//         if lesser > item {
//             lesser = item;
//         }
//     }

//     lesser
// }
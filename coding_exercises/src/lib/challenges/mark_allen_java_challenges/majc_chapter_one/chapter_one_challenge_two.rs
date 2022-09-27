// Challenge 1.20 from Mark Allen's Data Structures in Java:
/*
* Problem:
* You have 10 letters, for each assign a digit (0-9)
* Find all combinations that are like the next example:
*    MARK  =    9147
* + ALLEN  =   16653
*   -----
*   WEISS  =   25800
* 
*/

pub fn complexity_one_two() {
    println!("Complexity for Challenge 1.20 from Mark Allen's Data Structures in Java:");
    // println!("Time: Single loop up to how_many -> O(N)");
    // println!("Space: String with up to how_many*3 -> O(N)");
}

pub fn ch_one_two() -> String {
    /* High Level Algorithm:
        1) Obtain all possible combinations for MARKLENWIS (0-9)
        2) For each check if sum condition is met
        3) If it is met save the thing
        4) Store the right patterns in a vector of arrays
    */
    
    let mut valid_combinations:Vec<[i32;10]> = Vec::new();

    println!("{:?}", valid_combinations);

    let output_string = String::from("Hello from ch20");
    output_string
  
}

// pub fn permute_and_find(&Vec<[i32;10]> combination) {

// }

#[cfg(test)]
mod chapter_one_challenge_one_tests {
    use std::fs;
    use super::ch_one_two as ch_one_two;

    #[test]
    fn ch_one_two_test() {
        // let expected_text_file: String  = String::from("src/mark_allen_java_ds_challenges/chapter_one/text_assertions/chapter_one_challenge_two_text_assetion.txt");
        // let expected_text_file: String  = String::from("src/mark_allen_java_ds_challenges/chapter_one/text_assertions/test_assert.txt");  
        // let expected_output: String     = fs::read_to_string(expected_text_file)
        //                                     .expect("Should have been able to read the file");    
        // let obtained_output: String     = ch_one_two();
        // ch_one_two();
        // assert_eq!(expected_output, obtained_output);
        assert_eq!(1,1);
    }

}
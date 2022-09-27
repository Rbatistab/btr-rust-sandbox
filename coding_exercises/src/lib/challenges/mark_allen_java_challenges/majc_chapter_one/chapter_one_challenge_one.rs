// Challenge 1.19 from Mark Allen's Data Structures in Java:
/*
* Problem:
* Print numbers from 1 to N as '[1][2][3]...[N]'
* with a maximum line length without trimming
* a '[x]' block
*
* @param   how_many     Number of numbrs to print
* @param   line_length  Maximum line length
* @return  void
*/

pub fn complexity_one_one() {
    println!("Complexity for Challenge 1.19 from Mark Allen's Data Structures in Java:");
    println!("Time: Single loop up to how_many -> O(N)");
    println!("Space: String with up to how_many*3 -> O(N)");
}

pub fn ch_one_one(how_many: u32, line_length: u32) -> String {
    /* High Level Algorithm:
        1) Set a character counter
        2) Calculate the new block '[n]' size
        3) If it doesn't fit, print a new line and restart the character counter
        4) print the block
        */
    let mut output_string: String   = String::new();
    let mut character_counter: u32  = 0;
    let mut current_number: u32     = 1;

    while current_number <= how_many {
        let current_block: String           = String::from("[") + &current_number.to_string() + "]";
        let current_block_size: u32         = current_block.len()
                                                .to_string()
                                                .parse()
                                                .expect("Not a number");
        let block_fits_in_this_line: bool   = line_length <= (character_counter + current_block_size);

        if block_fits_in_this_line {
            output_string = output_string + "\n";
            character_counter = 0;
        }

        output_string = output_string + &current_block;

        character_counter+= current_block_size;
        current_number+= 1;
    }
    
    output_string
    
}

#[cfg(test)]
mod chapter_one_challenge_one_tests {
    use std::fs;
    use super::ch_one_one as ch_one_one;

    #[test]
    fn ch_one_one_test() {
        use crate::constants::MARK_ALLEN_JAVA_CHALLENGES_CHAPTER_ONE_ASSERTIONS as chapter_one_assertions_path;

        let how_many: u32           = 11;
        let line_length: u32        = 9;
        let mut expected_text_file: String = String::from(chapter_one_assertions_path);
        expected_text_file.push_str("/one_one_text_assertion.txt"); 
        let expected_output: String     =   fs::read_to_string(expected_text_file)
                                            .expect("Should have been able to read the file");    
        let obtained_output: String = ch_one_one(how_many, line_length);

        assert_eq!(expected_output, obtained_output)
    }
}
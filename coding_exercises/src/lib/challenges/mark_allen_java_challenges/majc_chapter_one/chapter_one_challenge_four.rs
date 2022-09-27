/**
* Programming Projects 1.22
* Problem:
* Write a program to take an integer parameter and return its equivalent in roman numbers
*/

pub fn complexity_one_four() {
    println!("Complexity for Challenge 1.22 from Mark Allen's Data Structures in Java:");
    println!("Time: Loops over the digits of the number (N) up to 4 digits, then for each reapeats pretty much same function -> O(1)");
    println!("Space: For each digit creates a string up to 4 characters, fixed value of 4 digits -> O(1)");
}

pub fn ch_one_four(latin_number: u32) -> String {
    // High level algorithm:
    // 1) Divide the number as single digits (up to the thousands)
    // 2) Get the roman equivalent of each digit acording to their power of 10
    // 3) Concatenate from thousands to units

    let mut roman_equivalent: String = String::new();
    let array_of_digits_from_latin_number: [u32;4] = get_digits_from_number(latin_number);
    let (thousands_digit, hundreds_digit, tens_digit, units_digit) = (
        array_of_digits_from_latin_number[0],
        array_of_digits_from_latin_number[1],
        array_of_digits_from_latin_number[2],
        array_of_digits_from_latin_number[3]
    );

    let roman_equvalent_of_thousands: String    = get_roman_equivalent_from_thousands(thousands_digit);
    let roman_equvalent_of_hundreds: String     = get_roman_equivalent_from_hundreds(hundreds_digit);
    let roman_equvalent_of_tens: String         = get_roman_equivalent_from_tens(tens_digit);
    let roman_equvalent_of_units: String        = get_roman_equivalent_from_units(units_digit);
    
    roman_equivalent = roman_equivalent 
        + &roman_equvalent_of_thousands
        + &roman_equvalent_of_hundreds 
        + &roman_equvalent_of_tens 
        + &roman_equvalent_of_units;

    roman_equivalent
}

fn get_roman_equivalent_from_units(units_digit: u32) -> String {
    let unit_character: char        = 'I'; 
    let middle_character: char      = 'V'; 
    let upper_magnitude_char: char  = 'X';
    let roman_units = roman_digit_string_generator(units_digit, unit_character, middle_character, upper_magnitude_char);

    roman_units
}

fn get_roman_equivalent_from_tens(tens_digit: u32) -> String {
    let unit_character: char        = 'X'; 
    let middle_character: char      = 'L'; 
    let upper_magnitude_char: char  = 'C';
    let roman_tens = roman_digit_string_generator(tens_digit, unit_character, middle_character, upper_magnitude_char);

    roman_tens
}

fn get_roman_equivalent_from_hundreds(hundreds_digit: u32) -> String {
    let unit_character: char        = 'C'; 
    let middle_character: char      = 'D'; 
    let upper_magnitude_char: char  = 'M';
    let roman_hundreds = roman_digit_string_generator(hundreds_digit, unit_character, middle_character, upper_magnitude_char);

    roman_hundreds
}

fn roman_digit_string_generator(digit: u32, unit_character: char, middle_character: char, upper_magnitude_char: char) -> String {
    let mut roman_numeral: String = String::new();
    
    if digit <= 4 {
        let mut _counter = 1;
        while digit <=3 {
            roman_numeral.push(unit_character);
            _counter += 1;
        }
    }
    if digit == 4 {
        roman_numeral.push(unit_character);
        roman_numeral.push(middle_character);
    }
    if digit == 5 {
        roman_numeral.push(middle_character);
    }
    if 5 < digit && digit < 9 {
        let mut _counter = 6;
        while digit <=3 {
            roman_numeral.push(unit_character);
            _counter += 1;
        }
    }
    if digit == 9 {
        roman_numeral.push(unit_character);
        roman_numeral.push(upper_magnitude_char);
    }


    roman_numeral
}

fn get_roman_equivalent_from_thousands(thousands_digit: u32) -> String {
    let mut roman_thousands = String::new();
    
    let mut counter = 1;
    while counter <= thousands_digit {
        roman_thousands.push('M');
        counter += 1;
    }

    roman_thousands
}

fn get_digits_from_number(number: u32) -> [u32; 4] {
    let thousands:u32   = number/1000;
    let hundreds:u32    = (number - thousands*1000)/100;
    let tens:u32        = (number - thousands*1000 - hundreds*100)/10;
    let units:u32       = number - thousands*1000 - hundreds*100 - tens*10;

    [thousands, hundreds, tens, units]
}

#[cfg(test)]
mod chapter_one_challenge_one_tests {
    use super::ch_one_four as ch_one_four;
    #[test]
    fn ch_one_four_test() {
        let decimal_test_number: u32        = 1994;
        let expected_roman_numeral: String  = String::from("MCMXCIV");
        let obtained_roman_numeral: String  = ch_one_four(decimal_test_number);

        assert_eq!(expected_roman_numeral, obtained_roman_numeral);
    }   
}
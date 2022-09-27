// Need help? https://github.com/YJDoc2/8086-Emulator/blob/master/src/lib/lib.rs
use cargo_challenges_lib as lib;
use lib::challenges::mark_allen_java_challenges::majc_chapter_one as majc_chapter_one;

fn main() {
    println!("Hello!");
    debug_current_challenge();
}

fn debug_current_challenge() {
    use majc_chapter_one::chapter_one_challenge_one::ch_one_one as mark_one_one;
    use majc_chapter_one::chapter_one_challenge_two::ch_one_two as mark_one_two;
    use majc_chapter_one::chapter_one_challenge_three::ch_one_three as mark_one_three;
    use majc_chapter_one::chapter_one_challenge_four::ch_one_four as mark_one_four;
    println!("Challenge #1: Expected to print up to 11 with lines no greater than 9 characters");
    println!( "{}", mark_one_one(11, 9) );
    println!("");

    println!("Challenge #2:");
    println!( "{}", mark_one_two() );
    println!("");

    println!("Challenge #3:");
    println!( "{:?}", mark_one_three() );
    println!("");

    println!("Challenge #4: Number = 1994 (expeted is MCMXCIV)");
    let number_lesser_than_three_thousand = 1994;
    let roman_number = mark_one_four(number_lesser_than_three_thousand);
    println!("The roman number for {} is: {}", number_lesser_than_three_thousand, roman_number);
    println!("");
}
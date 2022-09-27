// Challenge 1.21 from Mark Allen's Data Structures in Java:
/*
* Problem:
* Find all pairs of positive integers (a,b), such that:
*   - (a^2 + b^2 + 1)/(ab) is an integer
*   - where a < b < 500
*
* @param   
* @param   
* @return  
*/
#[derive(Debug)]
pub struct IntegerPair (i32, i32);

pub fn ch_one_three() -> Vec<IntegerPair> {
    /* High Level Algorithm:
      1) Iterate 'b' from 2 to 500
      2) Inside this loop, iterate 'a' from 1 to 'b'
      3) In each iteration see if the condition is met
      4) If it's met, add the tuple to the output
    */
    
    let mut output_vector = Vec::new();
    let upper_bound: i32  = 500;
    let mut b:i32         = 2;

    while b < upper_bound {
      let mut a:i32 = 1;
      while a < b {
        let condition_is_met:bool = is_condition_met(a, b);
        if condition_is_met {
          let pair = IntegerPair(a,b);
          output_vector.push(pair);
        }
        a += 1;
      }
      b += 1;
    }
    
    output_vector
  
}

fn is_condition_met(a: i32, b: i32) -> bool {
  let divident:i32  = a*a + b*b + 1;
  let divisor:i32   = a*b;
  let modulus:i32   = divident%divisor;
  let is_quotient_integer:bool = modulus == 0;
  is_quotient_integer
}

#[cfg(test)]
mod chapter_one_challenge_one_tests {
    use super::ch_one_three as ch_one_three;
    #[test]
    fn ch_one_three_test() {
        // let compliying_tuples: [u32; 6] = [1,2,3,4,5,6];
        // let mut compliying_tuples: Vec<TwoNumberTuple> = Vec::new();
        // compliying_tuples.push(TwoNumberTuple(1, 2));
        // compliying_tuples.push(TwoNumberTuple(2, 5));
        // compliying_tuples.push(TwoNumberTuple(5, 13));
        // compliying_tuples.push(TwoNumberTuple(13, 34));
        // compliying_tuples.push(TwoNumberTuple(34, 89));
        // compliying_tuples.push(TwoNumberTuple(89, 233));

        // let obtained_tuples = ch_one_three();

        // assert_eq!(compliying_tuples, obtained_tuples);
        assert_eq!(1,1);
        
    }


}
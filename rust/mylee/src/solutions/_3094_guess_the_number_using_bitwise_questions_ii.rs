// # [3094. Guess the Number Using Bitwise Questions II 🔒](https://leetcode.com/problems/guess-the-number-using-bitwise-questions-ii)

// ## Description

//

// There is a number n between 0 and 230 - 1 (both inclusive) that you have to find.

// There is a pre-defined API int commonBits(int num) that helps you with your mission.
// But here is the challenge, every time you call this function,
//  n changes in some way.
// But keep in mind, that you have to find the initial value of n.

// commonBits(int num) acts as follows:

//
// 	Calculate count which is the number of bits where both n and num have the same value in that position of their binary representation.
// 	n = n XOR num
// 	Return count.
//

// Return the number n.

// Note: In this world, all numbers are between 0 and 230 - 1 (both inclusive),
//  thus for counting common bits, we see only the first 30 bits of those numbers.

//
// Constraints:

//
// 	0 <= n <= 230 - 1
// 	0 <= num <= 230 - 1
// 	If you ask for some num out of the given range, the output wouldn't be reliable.
//

//     int find_number() {

//  * Definition of commonBits API.
//  * int commonBits(int num);
fn common_bits(num: i32) -> i32 {
    num
}
#[allow(dead_code)]
pub struct Solution;
/**
 * Definition of commonBits API.
 * unsafe fn common_bits(num: i32) -> i32 {}
 */

impl Solution {
    unsafe fn find_number() -> i32 {
        let mut ans = 0;
        let mut i = 1;
        while i < 1073741824 {
            if common_bits(i) > common_bits(i) {
                ans |= i;
            }

            i <<= 1;
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_find_number_1() {
        assert_eq!(0, unsafe { Solution::find_number() });
    }
    #[test]
    pub fn test_find_number_2() {
        assert_eq!(0, unsafe { Solution::find_number() });
    }
    #[test]
    pub fn test_find_number_3() {
        assert_eq!(0, unsafe { Solution::find_number() });
    }
}

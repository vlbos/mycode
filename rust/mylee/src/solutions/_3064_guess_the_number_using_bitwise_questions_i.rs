// # [3064. Guess the Number Using Bitwise Questions I](https://leetcode.com/problems/guess-the-number-using-bitwise-questions-i)

// ## Description

// There is a number n that you have to find.

// There is also a pre-defined API int commonSetBits(int num),
// which returns the number of bits where both n and num are 1 in that position of their binary representation.
// In other words, it returns the number of set bits in n & num, where & is the bitwise AND operator.

// Return the number n.

// Example 1:

// Input:   n = 31

// Output:   31

// Explanation:  It can be proven that it 's possible to find 31 using the provided API.

// Example 2:

// Input:   n = 33

// Output:   33

// Explanation:  It can be proven that it 's possible to find 33 using the provided API.

// Constraints:

// 	1  <= n  <= 230 - 1
// 	0  <= num  <= 230 - 1
// 	If you ask for some num out of the given range, the output wouldn 't be reliable.

// ```cpp
// /**
//  * Definition of commonSetBits API.
//  * int commonSetBits(int num);
//  */
fn common_set_bits(num: i32) -> i32 {
    0
}
// class Solution {
// public:
//     int find_number() {
#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn find_number() -> i32 {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_find_number_1() {
        assert_eq!(31, Solution::find_number());
    }
    #[test]
    pub fn test_find_number_2() {
        assert_eq!(33, Solution::find_number());
    }
}

// # [2533. Number of Good Binary Strings](https://leetcode.com/problems/number-of-good-binary-strings)
// ## Description

//  You are given four integers  minLength ,  maxLength ,  oneGroup  and  zeroGroup .

//  A binary string is  good  if it satisfies the following conditions:

// 	 The length of the string is in the range  [minLength, maxLength] .
// 	 The size of each block of consecutive  1 's is a multiple of  oneGroup .

// 		 For example in a binary string  00 11 0 1111 00  sizes of each block of consecutive ones are  [2,4] .

// 	 The size of each block of consecutive  0 's is a multiple of  zeroGroup .

// 		 For example, in a binary string   00 11 0 1111 00   sizes of each block of consecutive ones are  [2,1,2] .

//  Return  the number of  good  binary strings . Since the answer may be too large, return it  modulo   10^9  + 7 .

//   Note  that  0  is considered a multiple of all the numbers.

//  Example 1:

//  Input:  minLength = 2, maxLength = 3, oneGroup = 1, zeroGroup = 2
//  Output:  5
//  Explanation:  There are 5 good binary strings in this example: "00", "11", "001", "100", and "111".
// It can be proven that there are only 5 good strings satisfying all conditions.

//  Example 2:

//  Input:  minLength = 4, maxLength = 4, oneGroup = 4, zeroGroup = 3
//  Output:  1
//  Explanation:  There is only 1 good binary string in this example: "1111".
// It can be proven that there is only 1 good string satisfying all conditions.

//   Constraints:

// 	  1 <= minLength <= maxLength <= 10^5
// 	  1 <= oneGroup, zeroGroup <= maxLength
// int good_binary_strings(int min_length, int max_length, int one_group, int zero_group) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn good_binary_strings(
        min_length: i32,
        max_length: i32,
        one_group: i32,
        zero_group: i32,
    ) -> i32 {
        let m = 1_000_000_007;
        let mut f = vec![0; max_length as usize + 1];
        f[0] = 1;
        for i in 1..=max_length {
            if i - one_group >= 0 {
                f[i as usize] += f[(i - one_group) as usize];
                f[i as usize] %= m;
            }
            if i - zero_group >= 0 {
                f[i as usize] += f[(i - zero_group) as usize];
                f[i as usize] %= m;
            }
        }
        f[min_length as usize..]
            .iter()
            .fold(0, |mut a, x| (a + x) % m) as _
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_good_binary_strings_1() {
        assert_eq!(5, Solution::good_binary_strings(2, 3, 1, 2));
    }
    #[test]
    pub fn test_good_binary_strings_2() {
        assert_eq!(1, Solution::good_binary_strings(4, 4, 4, 3));
    }
}

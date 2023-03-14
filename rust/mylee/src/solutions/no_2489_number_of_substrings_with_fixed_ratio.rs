// # [2489. Number of Substrings With Fixed Ratio](https://leetcode.com/problems/number-of-substrings-with-fixed-ratio)
// ## Description

//  You are given a binary string  s , and two integers  num1  and  num2 .  num1  and  num2  are coprime numbers.
//  A  ratio substring  is a substring of s where the ratio between the number of  0 's
// and the number of  1 's in the substring is exactly  num1 : num2 .
// 	 For example, if  num1 = 2  and  num2 = 3 , then  "01011"  and  "1110000111"  are ratio substrings, while  "11000"  is not.
//  Return  the number of  non-empty  ratio substrings of   s .
//   Note  that:

// 	 A  substring  is a contiguous sequence of characters within a string.
// 	 Two values  x  and  y  are  coprime  if  gcd(x, y) == 1  where  gcd(x, y)  is the greatest common divisor of  x  and  y .

//  Example 1:

//  Input:  s = "0110011", num1 = 1, num2 = 2
//  Output:  4
//  Explanation:  There exist 4 non-empty ratio substrings.
// - The substring s[0..2]: " 011 0011". It contains one 0 and two 1's. The ratio is 1 : 2.
// - The substring s[1..4]: "0 110 011". It contains one 0 and two 1's. The ratio is 1 : 2.
// - The substring s[4..6]: "0110 011 ". It contains one 0 and two 1's. The ratio is 1 : 2.
// - The substring s[1..6]: "0 110011 ". It contains two 0's and four 1's. The ratio is 2 : 4 == 1 : 2.
// It can be shown that there are no more ratio substrings.

//  Example 2:

//  Input:  s = "10101", num1 = 3, num2 = 1
//  Output:  0
//  Explanation:  There is no ratio substrings of s. We return 0.

//   Constraints:

// 	  1 <= s.length <= 10^5
// 	  1 <= num1, num2 <= s.length
// 	  num1  and  num2  are coprime integers.
//   long long fixed_ratio(string s, int num1, int num2) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn fixed_ratio(s: String, num1: i32, num2: i32) -> i64 {
        let mut n = vec![0; 2];
        let mut ans = 0;
        let mut cnt = std::collections::HashMap::from([(0, 1)]);
        for b in s.bytes() {
            n[(b - b'0') as usize] += 1;
            let x = n[1] * num1 - n[0] * num2;
            ans += *cnt.get(&x).unwrap_or(&0);
            *cnt.entry(x).or_insert(0) += 1;
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_fixed_ratio_1() {
        assert_eq!(4, Solution::fixed_ratio("0110011".to_string(), 1, 2));
    }
    #[test]
    pub fn test_fixed_ratio_2() {
        assert_eq!(0, Solution::fixed_ratio("10101".to_string(), 3, 1));
    }
}

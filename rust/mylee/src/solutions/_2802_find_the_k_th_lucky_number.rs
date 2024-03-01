// # [2802. Find The K-th Lucky Number](https://leetcode.com/problems/find-the-k-th-lucky-number)

// ## Description

// We know that 4 and 7 are lucky digits. Also, a number is called lucky if it contains only lucky digits.

// You are given an integer k, return the kth lucky number represented as a string.

//
// ### Example 1:

//
// Input: k = 4
// Output: "47"
// Explanation: The first lucky number is 4, the second one is 7, the third one is 44 and the fourth one is 47.
//

// ### Example 2:

//
// Input: k = 10
// Output: "477"
// Explanation: Here are lucky numbers sorted in increasing order:
// 4, 7, 44, 47, 74, 77, 444, 447, 474, 477. So the 10th lucky number is 477.

// ### Example 3:

//
// Input: k = 1000
// Output: "777747447"
// Explanation: It can be shown that the 1000th lucky number is 777747447.
//

//
// Constraints:

//
// 	1 <= k <= 109
//

// ## Solutions

// ### **C++**

// ```cpp
// class Solution {
// public:
//     string kth_lucky_number(int k) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn kth_lucky_number(mut k: i32) -> String {
        let mut n = 1;
        while k > 1 << n {
            k -= 1 << n;
            n += 1;
        }
        let mut ans = String::new();
        while n > 0 {
            n -= 1;

            if k <= 1 << n {
                ans.push('4');
            } else {
                ans.push('7');
                k -= 1 << n;
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_kth_lucky_number_1() {
        assert_eq!("47".to_string(), Solution::kth_lucky_number(4));
    }
    #[test]
    pub fn test_kth_lucky_number_2() {
        assert_eq!("477".to_string(), Solution::kth_lucky_number(10));
    }
    #[test]
    pub fn test_kth_lucky_number_3() {
        assert_eq!("777747447".to_string(), Solution::kth_lucky_number(1000));
    }
}

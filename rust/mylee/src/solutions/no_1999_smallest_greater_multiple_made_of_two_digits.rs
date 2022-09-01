// 1999 - Smallest Greater Multiple Made of Two Digits[](https://leetcode.ca/2021-09-13-1999-Smallest-Greater-Multiple-Made-of-Two-Digits/#1999-smallest-greater-multiple-made-of-two-digits)
// ==========================================================================================================================================================================================

// Posted on September 13, 2021 · 1 minute read

// Formatted question description: [https://leetcode.ca/all/1999.html](https://leetcode.ca/all/1999.html)

// Description[](https://leetcode.ca/2021-09-13-1999-Smallest-Greater-Multiple-Made-of-Two-Digits/#description)
// ------------------------------------------------------------------------------------------------------------

// LeetCode Problem 1999.

// Given three integers, k, digit1, and digit2, you want to find the smallest integer that is:

// *   Larger than k,
// *   A multiple of k, and
// *   Comprised of only the digits digit1 and/or digit2.

// Return the smallest such integer. If no such integer exists or the integer exceeds the limit of a signed 32-bit integer (231 - 1), return -1.

// Example 1:

//     Input: k = 2, digit1 = 0, digit2 = 2
//     Output: 20
//     Explanation:
//     20 is the first integer larger than 2, a multiple of 2, and comprised of only the digits 0 and/or 2.

// Example 2:

//     Input: k = 3, digit1 = 4, digit2 = 2
//     Output: 24
//     Explanation:
//     24 is the first integer larger than 3, a multiple of 3, and comprised of only the digits 4 and/or 2.

// Example 3:

//     Input: k = 2, digit1 = 0, digit2 = 0
//     Output: -1
//     Explanation:
//     No integer meets the requirements so return -1.

// Constraints:

// *   1 <= k <= 1000
// *   0 <= digit1 <= 9
// *   0 <= digit2 <= 9

// Sample C++ Code[](https://leetcode.ca/2021-09-13-1999-Smallest-Greater-Multiple-Made-of-Two-Digits/#sample-c-code)
// ------------------------------------------------------------------------------------------------------------------

// We build our number x recursivelly using digit d1 or d2. Note that we cannot use a digit if it’s zero and x is zero.

// Two cases:

// *   x gets too big. Return -1.
// *   x is greater than k and divisible by k. Return x.

// If we get positive results for both digits, return the smallest one. For example, we can build 22 and 4 for k == 2 from digits 2 and 4. The result (smallest) is 4.

//     int find_integer(int k, int d1, int d2) {

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn find_integer(k: i32, d1: i32, d2: i32, x: i64) -> i32 {
        if x > i32::MAX as i64 {
            return -1;
        }
        let x = x as i32;
        if x > k && x % k == 0 {
            return x;
        }
        let x1 = if x + d1 == 0 {
            -1
        } else {
            Self::find_integer(k, d1, d2, x as i64 * 10 + d1 as i64)
        };
        let x2 = if x + d2 == 0 {
            -1
        } else {
            Self::find_integer(k, d1, d2, x as i64 * 10 + d2 as i64)
        };
        if x1 > 1 && x2 > 1 {
            x1.min(x2)
        } else {
            x1.max(x2)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_find_integer_1() {
        assert_eq!(20, Solution::find_integer(2, 0, 2, 0));
    }
    #[test]
    pub fn test_find_integer_2() {
        assert_eq!(24, Solution::find_integer(3, 4, 2, 0));
    }
    #[test]
    pub fn test_find_integer_3() {
        assert_eq!(-1, Solution::find_integer(2, 0, 0, 0));
    }
}

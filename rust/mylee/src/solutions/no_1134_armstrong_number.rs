// 1134\. Armstrong Number
// =======================

// The k-digit number `N` is an Armstrong number if and only if the k-th power of each digit sums to N.

// Given a positive integer `N`, return true if and only if it is an Armstrong number.

// **Example 1:**

// **Input:** 153
// **Output:** true
// **Explanation:**
// 153 is a 3-digit number, and 153 = 1^3 + 5^3 + 3^3.

// **Example 2:**

// **Input:** 123
// **Output:** false
// **Explanation:**
// 123 is a 3-digit number, and 123 != 1^3 + 2^3 + 3^3 = 36.

// **Note:**

// 1.  `1 <= N <= 10^8`

// ### Difficulty:

// Easy

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon)

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn   is_armstrong(n: i32) -> bool {
        let ns = n.to_string();
        ns.bytes()
            .map(|b| ((b - b'0') as i32).pow(ns.len() as u32))
            .sum::<i32>()
            == n
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
   pub fn  test_is_armstrong_1() {
        assert!(Solution::is_armstrong(153));
    }

    #[test]
   pub fn  test_is_armstrong_2() {
        assert!(!Solution::is_armstrong(123));
    }
}

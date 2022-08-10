// 1085\. Sum of Digits in the Minimum Number
// ==========================================

// Given an array `A` of positive integers, let `S` be the sum of the digits of the minimal element of `A`.

// Return 0 if `S` is odd, otherwise return 1.

// **Example 1:**

// **Input:** \[34,23,1,24,75,33,54,8\]
// **Output:** 0
// **Explanation:**
// The minimal element is 1, and the sum of those digits is S = 1 which is odd, so the answer is 0.

// **Example 2:**

// **Input:** \[99,77,33,66,55\]
// **Output:** 1
// **Explanation:**
// The minimal element is 33, and the sum of those digits is S = 3 + 3 = 6 which is even, so the answer is 1.

// **Note:**

// 1.  `1 <= A.length <= 100`
// 2.  `1 <= A[i].length <= 100`

// ### Difficulty:

// Easy

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon)
 
#[allow(dead_code)] 
 pub struct Solution {}
impl Solution {
    pub fn sum_of_digits(a: Vec<i32>) -> i32 {
        1 - a
            .into_iter()
            .min()
            .unwrap()
            .to_string()
            .bytes()
            .map(|x| (x - b'0') as i32)
            .sum::<i32>()
            % 2
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sum_of_digits_1() {
        assert_eq!(
            0,
            Solution::sum_of_digits(vec![34, 23, 1, 24, 75, 33, 54, 8])
        );
    }
    #[test]
    fn test_sum_of_digits_2() {
        assert_eq!(1, Solution::sum_of_digits(vec![99, 77, 33, 66, 55]));
    }
}

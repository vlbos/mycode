// 1063\. Number of Valid Subarrays
// ================================

// Given an array `A` of integers, return the number of **non-empty continuous subarrays** that satisfy the following condition:

// The leftmost element of the subarray is not larger than other elements in the subarray.

// **Example 1:**

// **Input:** \[1,4,2,5,3\]
// **Output:** 11
// **Explanation:** There are 11 valid subarrays: \[1\],\[4\],\[2\],\[5\],\[3\],\[1,4\],\[2,5\],\[1,4,2\],\[2,5,3\],\[1,4,2,5\],\[1,4,2,5,3\].

// **Example 2:**

// **Input:** \[3,2,1\]
// **Output:** 3
// **Explanation:** The 3 valid subarrays are: \[3\],\[2\],\[1\].

// **Example 3:**

// **Input:** \[2,2,2\]
// **Output:** 6
// **Explanation:** There are 6 valid subarrays: \[2\],\[2\],\[2\],\[2,2\],\[2,2\],\[2,2,2\].

// **Note:**

// 1.  `1 <= A.length <= 50000`
// 2.  `0 <= A[i] <= 100000`

// ### Difficulty:

// Hard

// ### Lock:

// Prime

// ### Company:

// [Hulu](https://leetcode.ca/tags/#Hulu)

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn   valid_subarrays(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut ans = 0;
        let mut s = Vec::new();
        for (i, &num) in nums.iter().enumerate() {
            while !s.is_empty() && nums[s[s.len() - 1]] > num {
                ans += (i - s.pop().unwrap()) as i32;
            }
            s.push(i);
        }
        while !s.is_empty() {
            ans += (n - s.pop().unwrap()) as i32;
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
   pub fn  test_valid_subarrays_1() {
        assert_eq!(11, Solution::valid_subarrays(vec![1, 4, 2, 5, 3]));
    }
    #[test]
   pub fn  test_valid_subarrays_2() {
        assert_eq!(3, Solution::valid_subarrays(vec![3, 2, 1]));
    }
    #[test]
   pub fn  test_valid_subarrays_3() {
        assert_eq!(6, Solution::valid_subarrays(vec![2, 2, 2]));
    }
}

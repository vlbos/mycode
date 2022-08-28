// 1852\. Distinct Numbers in Each Subarray
// ========================================

// Given an integer array `nums` and an integer `k`, you are asked to construct the array `ans` of size `n-k+1` where `ans[i]` is the number of **distinct** numbers in the subarray `nums[i:i+k-1] = [nums[i], nums[i+1], ..., nums[i+k-1]]`.

// Return _the array_ `ans`.

// **Example 1:**

// **Input:** nums = \[1,2,3,2,2,1,3\], k = 3
// **Output:** \[3,2,2,2,3\]
// **Explanation:** The number of distinct elements in each subarray goes as follows:
// - nums\[0:2\] = \[1,2,3\] so ans\[0\] = 3
// - nums\[1:3\] = \[2,3,2\] so ans\[1\] = 2
// - nums\[2:4\] = \[3,2,2\] so ans\[2\] = 2
// - nums\[3:5\] = \[2,2,1\] so ans\[3\] = 2
// - nums\[4:6\] = \[2,1,3\] so ans\[4\] = 3

// **Example 2:**

// **Input:** nums = \[1,1,1,1,2,3,4\], k = 4
// **Output:** \[1,2,3,4\]
// **Explanation:** The number of distinct elements in each subarray goes as follows:
// - nums\[0:3\] = \[1,1,1,1\] so ans\[0\] = 1
// - nums\[1:4\] = \[1,1,1,2\] so ans\[1\] = 2
// - nums\[2:5\] = \[1,1,2,3\] so ans\[2\] = 3
// - nums\[3:6\] = \[1,2,3,4\] so ans\[3\] = 4

// **Constraints:**

// *   `1 <= k <= nums.length <= 105`
// *   `1 <= nums[i] <= 105`

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Unknown](https://leetcode.ca/tags/#Unknown)

// int[] distinct_numbers(int[] nums, int k) {

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn distinct_numbers(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if k == 1 {
            return nums;
        }
        let k = k as usize;
        let mut s = std::collections::HashMap::new();
        for (i, &v) in nums[..k].iter().enumerate() {
            s.insert(v, i);
        }
        let mut ans = vec![s.len() as i32];
        for i in k..nums.len() {
            if *s.get(&nums[i - k]).unwrap() <= i - k {
                s.remove(&nums[i - k]);
            }
            s.insert(nums[i], i);
            ans.push(s.len() as i32);
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_distinct_numbers_1() {
        assert_eq!(
            vec![3, 2, 2, 2, 3],
            Solution::distinct_numbers(vec![1, 2, 3, 2, 2, 1, 3], 3)
        );
    }
    #[test]
    pub fn test_distinct_numbers_2() {
        assert_eq!(
            vec![1, 2, 3, 4],
            Solution::distinct_numbers(vec![1, 1, 1, 1, 2, 3, 4], 4)
        );
    }
}

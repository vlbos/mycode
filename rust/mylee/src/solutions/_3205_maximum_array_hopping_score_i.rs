// # [3205. Maximum Array Hopping Score I 🔒](https://leetcode.com/problems/maximum-array-hopping-score-i)

// ## Description

//

// Given an array nums,
// you have to get the maximum score starting from index 0 and hopping until you reach the last element of the array.

// In each hop, you can jump from index i to an index j  > i, and you get a score of (j - i) * nums[j].

// Return the maximum score you can get.

//
// Example 1:

//
// Input: nums = [1,5,8]

// Output: 16

// Explanation:

// There are two possible ways to reach the last element:

//
// 	0 - > 1 - > 2 with a score of (1 - 0) * 5 + (2 - 1) * 8 = 13.
// 	0 - > 2 with a score of (2 - 0) * 8 = 16.
//
//

// Example 2:

//
// Input: nums = [4,5,2,8,9,1,3]

// Output: 42

// Explanation:

// We can do the hopping 0 - > 4 - > 6 with a score of (4 - 0) * 9 + (6 - 4) * 3 = 42.
//

//
// Constraints:

//
// 	2  <= nums.length  <= 103
// 	1  <= nums[i]  <= 105
//

//     int max_score(vector<int>& nums) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn max_score(nums: Vec<i32>) -> i32 {
        fn dfs(i: usize, nums: &Vec<i32>, f: &mut Vec<i32>) -> i32 {
            if f[i] > 0 {
                return f[i];
            }
            for j in i + 1..nums.len() {
                f[i] = f[i].max(nums[j] * (j - i) as i32 + dfs(j, nums, f));
            }
            f[i]
        }
        let mut f = vec![0; nums.len()];
        dfs(0, &nums, &mut f)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_max_score_1() {
        assert_eq!(16, Solution::max_score(vec![1, 5, 8]));
    }
    #[test]
    pub fn test_max_score_2() {
        assert_eq!(42, Solution::max_score(vec![4, 5, 2, 8, 9, 1, 3]));
    }
}

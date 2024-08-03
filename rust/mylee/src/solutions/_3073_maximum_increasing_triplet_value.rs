// # [3073. Maximum Increasing Triplet Value 🔒](https://leetcode.com/problems/maximum-increasing-triplet-value)

// ## Description

//

// Given an array nums, return the maximum value of a triplet (i, j, k) such that i < j < k and nums[i] < nums[j] < nums[k].

// The value of a triplet (i, j, k) is nums[i] - nums[j] + nums[k].

//
// Example 1:

//
// Input:  nums = [5,6,9]

// Output:  8

// Explanation:  We only have one choice for an increasing triplet and that is choosing all three elements.
// The value of this triplet would be 5 - 6 + 9 = 8.
//

// Example 2:

//
// Input:  nums = [1,5,3,6]

// Output:  4

// Explanation:  There are only two increasing triplets:

// (0, 1, 3): The value of this triplet is nums[0] - nums[1] + nums[3] = 1 - 5 + 6 = 2.

// (0, 2, 3): The value of this triplet is nums[0] - nums[2] + nums[3] = 1 - 3 + 6 = 4.

// Thus the answer would be 4.
//

//
// Constraints:

//
// 	3 <= nums.length <= 105
// 	1 <= nums[i] <= 109
// 	The input is generated such that at least one triplet meets the given condition.
//

// int maximum_triplet_value(vector<int>& nums) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut right = vec![nums[n - 1]; n];
        for (i, &x) in nums.iter().enumerate().rev().skip(1) {
            right[i] = right[i + 1].max(x);
        }
        let mut ans = 0;
        let mut sl = std::collections::BTreeSet::from([nums[0]]);
        for j in 1..n - 1 {
            if right[j + 1] > nums[j] {
                let i = sl.range(..nums[j]).next_back();
                if let Some(i) = i {
                    ans = ans.max(i - nums[j] + right[j + 1]);
                }
            }
            sl.insert(nums[j]);
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_maximum_triplet_value_1() {
        assert_eq!(8, Solution::maximum_triplet_value(vec![5, 6, 9]));
    }
    #[test]
    pub fn test_maximum_triplet_value_2() {
        assert_eq!(4, Solution::maximum_triplet_value(vec![1, 5, 3, 6]));
    }
}

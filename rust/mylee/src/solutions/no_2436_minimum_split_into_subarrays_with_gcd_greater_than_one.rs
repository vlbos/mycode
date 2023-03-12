// # [2436. Minimum Split Into Subarrays With GCD Greater Than One](https://leetcode.com/problems/minimum-split-into-subarrays-with-gcd-greater-than-one)
// ## Description

//  You are given an array  nums  consisting of positive integers.

//  Split the array into  one or more  disjoint subarrays such that:

// 	 Each element of the array belongs to  exactly one  subarray, and
// 	 The  GCD  of the elements of each subarray is strictly greater than  1 .

//  Return  the minimum number of subarrays that can be obtained after the split .

//   Note  that:

// 	 The  GCD  of a subarray is the largest positive integer that evenly divides all the elements of the subarray.
// 	 A  subarray  is a contiguous part of the array.

//  Example 1:

//  Input:  nums = [12,6,3,14,8]
//  Output:  2
//  Explanation:  We can split the array into the subarrays: [12,6,3] and [14,8].
// - The GCD of 12, 6 and 3 is 3, which is strictly greater than 1.
// - The GCD of 14 and 8 is 2, which is strictly greater than 1.
// It can be shown that splitting the array into one subarray will make the GCD = 1.

//  Example 2:

//  Input:  nums = [4,12,6,14]
//  Output:  1
//  Explanation:  We can split the array into only one subarray, which is the whole array.

//   Constraints:

// 	  1 <= nums.length <= 2000
// 	  2 <= nums[i] <= 10^9

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn minimum_splits(nums: Vec<i32>) -> i32 {
        let (mut i, mut j) = (0, nums.len() - 1);
        let (mut a, mut b) = (nums[i], nums[j]);
        let mut ans = 0;
        while i < j {
            if a < b {
                i += 1;
                a += nums[i];
                ans += 1;
            } else if a > b {
                j -= 1;
                b += nums[j];
                ans += 1;
            } else {
                i += 1;
                j -= 1;
                a = nums[i];
                b = nums[j];
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_minimum_splits_1() {
        assert_eq!(2, Solution::minimum_splits(vec![12, 6, 3, 14, 8]));
    }
    #[test]
    pub fn test_minimum_splits_2() {
        assert_eq!(1, Solution::minimum_splits(vec![4, 12, 6, 14]));
    }
}

// # [2505. Bitwise OR of All Subsequence Sums](https://leetcode.com/problems/bitwise-or-of-all-subsequence-sums)
// ## Description

//  Given an integer array  nums ,
// return  the value of the bitwise   OR   of the sum of all possible  subsequences  in the array .

//  A  subsequence  is a sequence that can be derived from another sequence
// by removing zero or more elements without changing the order of the remaining elements.

//  Example 1:

//  Input:  nums = [2,1,0,3]
//  Output:  7
//  Explanation:  All possible subsequence sums that we can have are: 0, 1, 2, 3, 4, 5, 6.
// And we have 0 OR 1 OR 2 OR 3 OR 4 OR 5 OR 6 = 7, so we return 7.

//  Example 2:

//  Input:  nums = [0,0,0]
//  Output:  0
//  Explanation:  0 is the only possible subsequence sum we can have, so we return 0.

//   Constraints:

// 	  1 <= nums.length <= 10^5
// 	  0 <= nums[i] <= 10^9
// long long subsequence_sum_or(vector<int>& nums) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn subsequence_sum_or(nums: Vec<i32>) -> i64 {
        let mut cnt = vec![0; 64];
        let mut ans = 0;
        for &v in &nums {
            for i in 0..31 {
                if v >> i & 1 > 0 {
                    cnt[i] += 1;
                }
            }
        }
        for i in 0..63 {
            if cnt[i] > 0 {
                ans |= 1 << i;
            }
            cnt[i + 1] += cnt[i] / 2;
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_subsequence_sum_or_1() {
        assert_eq!(7, Solution::subsequence_sum_or(vec![2, 1, 0, 3]));
    }
    #[test]
    pub fn test_subsequence_sum_or_2() {
        assert_eq!(0, Solution::subsequence_sum_or(vec![0, 0, 0]));
    }
}

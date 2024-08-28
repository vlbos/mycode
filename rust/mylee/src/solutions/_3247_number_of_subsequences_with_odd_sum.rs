// # [3247. Number of Subsequences with Odd Sum 🔒](https://leetcode.com/problems/number-of-subsequences-with-odd-sum)

// ## Description

// Given an array nums, return the number of subsequences with an odd sum of elements.

// Since the answer may be very large, return it modulo 109 + 7.

//
// Example 1:

//
// Input: nums = [1,1,1]

// Output: 4

// Explanation:

// The odd-sum subsequences are: [1, 1, 1], [1, 1, 1], [1, 1, 1], [1, 1, 1].
//

// Example 2:

//
// Input: nums = [1,2,2]

// Output: 4

// Explanation:

// The odd-sum subsequences are: [1, 2, 2], [1, 2, 2], [1, 2, 2], [1, 2, 2].
//

//
// Constraints:

//
// 	1  <= nums.lnegth  <= 105
// 	1  <= nums[i]  <= 109
//

//     int subsequence_count(vector<int>& nums) {
#[allow(dead_code)]
pub struct Solution;
impl Solution {
    pub fn subsequence_count(nums: Vec<i32>) -> i32 {
        if nums.iter().all(|&x| x % 2 == 0) {
            return 0;
        }
        let pow = |mut x: i64, mut n: i32| {
            let mut ans = 1;
            while n > 0 {
                if n & 1 != 0 {
                    ans = ans * x % 1_000_000_007;
                }
                x = x * x % 1_000_000_007;
                n >>= 1;
            }
            ans
        };
        pow(2, nums.len() as i32 - 1) as _
    }
    pub fn subsequence_count2(nums: Vec<i32>) -> i32 {
        let (mut odd, mut even) = (0i64, 0);
        for x in nums {
            if x % 2 == 0 {
                odd *= 2;
                even += even + 1;
            } else {
                (odd, even) = (
                    (odd + even + 1) % 1_000_000_007,
                    (even + odd) % 1_000_000_007,
                );
            }
        }
        (odd % 1_000_000_007) as _
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_subsequence_count_1() {
        assert_eq!(4, Solution::subsequence_count(vec![1, 1, 1]));
    }
    #[test]
    pub fn test_subsequence_count_2() {
        assert_eq!(4, Solution::subsequence_count(vec![1, 2, 2]));
    }
}

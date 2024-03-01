// # [2198. Number of Single Divisor Triplets](https://leetcode.com/problems/number-of-single-divisor-triplets)

// ## Description

// You are given a 0-indexed array of positive integers nums.
//  A triplet of three distinct indices (i, j, k) is called a single divisor triplet of nums if nums[i] + nums[j] + nums[k] is divisible by exactly one of nums[i], nums[j], or nums[k].
// Return the number of single divisor triplets of nums.

// Example 1:

//
// Input: nums = [4,6,7,3,2]
// Output: 12
// Explanation:
// The triplets (0, 3, 4), (0, 4, 3), (3, 0, 4), (3, 4, 0), (4, 0, 3), and (4, 3, 0) have the values of [4, 3, 2] (or a permutation of [4, 3, 2]).
// 4 + 3 + 2 = 9 which is only divisible by 3, so all such triplets are single divisor triplets.
// The triplets (0, 2, 3), (0, 3, 2), (2, 0, 3), (2, 3, 0), (3, 0, 2), and (3, 2, 0) have the values of [4, 7, 3] (or a permutation of [4, 7, 3]).
// 4 + 7 + 3 = 14 which is only divisible by 7, so all such triplets are single divisor triplets.
// There are 12 single divisor triplets in total.
//

// Example 2:

//
// Input: nums = [1,2,2]
// Output: 6
// Explanation:
// The triplets (0, 1, 2), (0, 2, 1), (1, 0, 2), (1, 2, 0), (2, 0, 1), and (2, 1, 0) have the values of [1, 2, 2] (or a permutation of [1, 2, 2]).
// 1 + 2 + 2 = 5 which is only divisible by 1, so all such triplets are single divisor triplets.
// There are 6 single divisor triplets in total.
//

// Example 3:

//
// Input: nums = [1,1,1]
// Output: 0
// Explanation:
// There are no single divisor triplets.
// Note that (0, 1, 2) is not a single divisor triplet because nums[0] + nums[1] + nums[2] = 3 and 3 is divisible by nums[0], nums[1], and nums[2].
//

// Constraints:

//
// 	3 <= nums.length <= 10^5
// 	1 <= nums[i] <= 100
//

// ## Solutions

// <!-- tabs:start -->

// ### **Python3**

// ```python
// class Solution:
//     def single_divisor_triplet(self, nums: List[int]) -> int:

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn single_divisor_triplet(nums: Vec<i32>) -> i64 {
        let n = 101;
        let mut count = vec![0; n];
        for &num in &nums {
            count[num as usize] += 1;
        }
        let mut ans = 0;
        for a in 1..n {
            for b in a..n {
                for c in b..n {
                    let sum = a + b + c;
                    if [sum % a, sum % b, sum % c]
                        .iter()
                        .filter(|&x| *x == 0)
                        .count()
                        != 1
                    {
                        continue;
                    }
                    if a == b {
                        ans += count[a] * (count[a] - 1) / 2 * count[c];
                    } else if b == c {
                        ans += count[b] * (count[b] - 1) / 2 * count[a];
                    } else {
                        ans += count[a] * count[b] * count[c];
                    }
                }
            }
        }
        ans * 6
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_single_divisor_triplet_1() {
        assert_eq!(12, Solution::single_divisor_triplet(vec![4, 6, 7, 3, 2]));
    }
    #[test]
    pub fn test_single_divisor_triplet_2() {
        assert_eq!(6, Solution::single_divisor_triplet(vec![1, 2, 2]));
    }
    #[test]
    pub fn test_single_divisor_triplet_3() {
        assert_eq!(0, Solution::single_divisor_triplet(vec![1, 1, 1]));
    }
}

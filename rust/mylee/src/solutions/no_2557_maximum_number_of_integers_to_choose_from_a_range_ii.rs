// # [2557. Maximum Number of Integers to Choose From a Range II](https://leetcode.com/problems/maximum-number-of-integers-to-choose-from-a-range-ii)

// You are given an integer array banned and two integers n and maxSum.
// You are choosing some number of integers following the below rules:

// 	The chosen integers have to be in the range [1, n].
// 	Each integer can be chosen at most once.
// 	The chosen integers should not be in the array banned.
// 	The sum of the chosen integers should not exceed maxSum.

// Return the maximum number of integers you can choose following the mentioned rules.

// Example 1:

// Input: banned = [1,4,6], n = 6, maxSum = 4
// Output: 1
// Explanation: You can choose the integer 3.
// 3 is in the range [1, 6], and do not appear in banned. The sum of the chosen integers is 3, which does not exceed maxSum.

// Example 2:

// Input: banned = [4,3,5,6], n = 7, maxSum = 18
// Output: 3
// Explanation: You can choose the integers 1, 2, and 7.
// All these integers are in the range [1, 7], all do not appear in banned, and their sum is 18, which does not exceed maxSum.

// Constraints:

// 	1 <= banned.length <= 10^5
// 	1 <= banned[i] <= n <= 10^9
// 	1 <= maxSum <= 10^15
//   int max_count(vector<int>& banned, int n, long long max_sum) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn max_count(mut banned: Vec<i32>, n: i32, mut max_sum: i64) -> i32 {
        banned.extend(vec![0, n + 1]);
        banned.sort();
        let mut ans = 0;
        for w in banned.windows(2) {
            let (l, r) = (w[0] as i64, w[1] as i64);
            let (mut left, mut right) = (0, r - l - 1);
            while left < right {
                let mid = (left + right + 1) / 2;
                if (l + 1 + l + mid) * mid / 2 <= max_sum {
                    left = mid;
                } else {
                    right = mid - 1;
                }
            }
            ans += left;
            max_sum -= (l + 1 + l + left) * left / 2;
            if max_sum <= 0 {
                break;
            }
        }
        ans as _
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_max_count_1() {
        assert_eq!(1, Solution::max_count(vec![1, 4, 6], 6, 4));
    }
    #[test]
    pub fn test_max_count_2() {
        assert_eq!(3, Solution::max_count(vec![4, 3, 5, 6], 7, 18));
    }
}

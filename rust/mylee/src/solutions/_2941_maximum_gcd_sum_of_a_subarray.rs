// # [2941. Maximum GCD-Sum of a Subarray](https://leetcode.com/problems/maximum-gcd-sum-of-a-subarray)

// ## Description

// You are given an array of integers nums and an integer k.

// The gcd-sum of an array a is calculated as follows:

// 	Let s be the sum of all the elements of a.
// 	Let g be the greatest common divisor of all the elements of a.
// 	The gcd-sum of a is equal to s * g.

// Return the maximum gcd-sum of a subarray of nums with at least k elements.

//
// Example 1:

// Input: nums = [2,1,4,4,4,2], k = 2
// Output: 48
// Explanation: We take the subarray [4,4,4], the gcd-sum of this array is 4 * (4 + 4 + 4) = 48.
// It can be shown that we can not select any other subarray with a gcd-sum greater than 48.

// Example 2:

// Input: nums = [7,3,9,4], k = 1
// Output: 81
// Explanation: We take the subarray [9], the gcd-sum of this array is 9 * 9 = 81.
// It can be shown that we can not select any other subarray with a gcd-sum greater than 81.

//
// Constraints:

// 	n == nums.length
// 	1  <= n  <= 105
// 	1  <= nums[i]  <= 106
// 	1  <= k  <= n

//     long long max_gcd_sum(vector<int>& nums, int k) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn max_gcd_sum(nums: Vec<i32>, k: i32) -> i64 {
        fn gcd(a: i32, b: i32) -> i32 {
            if b == 0 {
                a
            } else {
                gcd(b, a % b)
            }
        }
        let mut s: Vec<_> = nums
            .iter()
            .scan(0, |mut sum, &x| {
                *sum += x;
                Some(*sum)
            })
            .collect();
        s.insert(0, 0);
        let mut f = vec![];
        let mut ans = 0;
        for (i, v) in nums.into_iter().enumerate() {
            let mut g: Vec<(usize, i32)> = vec![];
            for &(j, x) in &f {
                let y = gcd(x, v);
                if g.is_empty() || g.last().unwrap().1 != y {
                    g.push((j, y));
                }
            }
            f = g;
            f.push((i, v));
            for &(j, x) in &f {
                if i - j + 1 >= k as usize {
                    ans = ans.max((s[i + 1] - s[j]) as i64 * x as i64);
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_max_gcd_sum_1() {
        assert_eq!(48, Solution::max_gcd_sum(vec![2, 1, 4, 4, 4, 2], 2));
    }
    #[test]
    pub fn test_max_gcd_sum_2() {
        assert_eq!(81, Solution::max_gcd_sum(vec![7, 3, 9, 4], 1));
    }
}

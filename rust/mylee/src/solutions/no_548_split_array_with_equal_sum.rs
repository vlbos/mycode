// 548\. Split Array with Equal Sum
// ================================

// Given an array with n integers, you need to find if there are triplets (i, j, k) which satisfies following conditions:

// 1.  0 < i, i + 1 < j, j + 1 < k < n - 1
// 2.  Sum of subarrays (0, i - 1), (i + 1, j - 1), (j + 1, k - 1) and (k + 1, n - 1) should be equal.

// where we define that subarray (L, R) represents a slice of the original array starting from the element indexed L to the element indexed R.

// **Example:**

// **Input:** \[1,2,1,2,1,2,1\]
// **Output:** True
// **Explanation:**
// i = 1, j = 3, k = 5.
// sum(0, i - 1) = sum(0, 0) = 1
// sum(i + 1, j - 1) = sum(2, 2) = 1
// sum(j + 1, k - 1) = sum(4, 4) = 1
// sum(k + 1, n - 1) = sum(6, 6) = 1

// **Note:**

// 1.  1 <= n <= 2000.
// 2.  Elements in the given array will be in range \[-1,000,000, 1,000,000\].

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Alibaba](https://leetcode.ca/tags/#Alibaba)

// @lc code=start

impl Solution {
    pub fn split_array(nums: Vec<i32>) -> bool {
        // use std::collections::HashMap;

        // let len = nums.len();
        // if len < 7 {
        //     return false;
        // }
        // let mut sum = 0;
        // let mut sum_dp = vec![0; len];
        // let mut dict = HashMap::<i32, Vec<usize>>::new();
        // for i in 0..len {
        //     let n = nums[i];
        //     sum_dp[i] = sum;
        //     sum += n;
        //     dict.entry(n)
        //         .and_modify(|v| v.push(i))
        //         .or_insert_with(|| vec![i]);
        // }
        // let mut i = 1;
        // while i + 6 <= len {
        //     let mut k = len - 2;
        //     while i + 4 <= k {
        //         let left_sum = sum_dp[i];
        //         let right_sum = sum - sum_dp[k] - nums[k];
        //         let mid_sum = sum_dp[k] - sum_dp[i] - nums[i];
        //         if left_sum == right_sum {
        //             let n_j = mid_sum - left_sum - right_sum;
        //             if let Some(js) = dict.get(&n_j) {
        //                 for &j in js {
        //                     if i + 1 < j && j + 1 < k {
        //                         return true;
        //                     }
        //                 }
        //             }
        //         }
        //         k -= 1;
        //     }
        //     i += 1;
        // }
        // false
        let n = nums.len();
        if n < 7 {
            return false;
        }
        let sums: Vec<i32> = nums
            .iter()
            .scan(0, |acc, &x| {
                *acc = *acc + x;
                Some(*acc)
            })
            .collect();
        for j in 3..n - 3 {
            let mut s = std::collections::HashSet::new();
            for i in 1..j - 1 {
                if sums[i - 1] == sums[j - 1] - sums[i] {
                    s.insert(sums[i - 1]);
                }
            }
            for k in j + 1..n - 1 {
                let (s3, s4) = (sums[k - 1] - sums[j], sums[n - 1] - sums[k]);
                if s3 == s4 && s.contains(&s3) {
                    return true;
                }
            }
        }
        false
    }
}
// @lc code=end

#[allow(dead_code)]
pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_split_array_1() {
        assert_eq!(Solution::split_array(vec![1, 2, 1, 2, 1, 2, 1]), true);
    }

    #[test]
    fn test_split_array_2() {
        assert_eq!(Solution::split_array(vec![-1, 0, -1, 0, -1, 0, -1]), true);
    }
}

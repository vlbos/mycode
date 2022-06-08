/*
 * @lc app=leetcode id=1671 lang=rust
 *
 * [1671] Minimum Number of Removals to Make Mountain Array
 */

// @lc code=start
impl Solution {
    pub fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let (mut left, mut right) = (vec![0; n], vec![0; n]);
        let mut dp = Vec::new();
        for (i, &v) in nums.iter().enumerate() {
            let (mut l, mut r) = (-1, dp.len() as i32);
            while l + 1 != r {
                let mid = (l + r) / 2;
                if dp[mid as usize] < v {
                    l = mid;
                } else {
                    r = mid;
                }
            }
            if r == dp.len() as i32 {
                dp.push(v);
                left[i] = dp.len() as i32;
            } else {
                dp[r as usize] = v;
                left[i] = r + 1;
            }
        }
        dp.clear();
        for (i, &v) in nums.iter().enumerate().rev() {
            let (mut l, mut r) = (-1, dp.len() as i32);
            while l + 1 != r {
                let mid = (l + r) / 2;
                if dp[mid as usize] < v {
                    l = mid;
                } else {
                    r = mid;
                }
            }
            if r == dp.len() as i32 {
                dp.push(v);
                right[i] = dp.len() as i32;
            } else {
                dp[r as usize] = v;
                right[i] = r + 1;
            }
        }
        let mut max_len = 0;
        for i in 0..n {
            if left[i] > 1 && right[i] > 1 {
                max_len = max_len.max(left[i] + right[i] - 1);
            }
        }
        n as i32 - max_len
    }
}
// @lc code=end

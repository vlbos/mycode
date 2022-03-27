/*
 * @lc app=leetcode id=2100 lang=rust
 *
 * [2100] Find Good Days to Rob the Bank
 */

// @lc code=start
impl Solution {
    pub fn good_days_to_rob_bank(security: Vec<i32>, time: i32) -> Vec<i32> {
        let n = security.len();
        if (n as i32) < time * 2 + 1 {
            return Vec::new();
        }
        let (mut left, mut right) = (vec![0; n], vec![0; n]);
        for i in 1..n {
            if security[i] <= security[i - 1] {
                left[i] = left[i - 1] + 1;
            }
            if security[n - i - 1] <= security[n - i] {
                right[n - i - 1] = right[n - i] + 1;
            }
        }
        let t = time as usize;
        (t..n - t)
            .filter(|&i| left[i] >= time && right[i] >= time)
            .map(|i| i as i32)
            .collect::<Vec<i32>>()
    }
}
// @lc code=end

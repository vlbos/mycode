/*
 * @lc app=leetcode id=1011 lang=rust
 *
 * [1011] Capacity To Ship Packages Within D Days
 */

// @lc code=start
impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        let mut left = *weights.iter().max().unwrap();
        let mut right = weights.iter().sum::<i32>();
        while left < right {
            let mid = (left + right) / 2;
            let mut cur = 0;
            let mut need = 0;
            for &w in &weights {
                if cur + w > mid {
                    need += 1;
                    cur = 0;
                }
                cur += w;
            }
            if need < days {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left
    }
}
// @lc code=end

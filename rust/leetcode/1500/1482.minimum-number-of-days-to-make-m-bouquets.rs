/*
 * @lc app=leetcode id=1482 lang=rust
 *
 * [1482] Minimum Number of Days to Make m Bouquets
 */

// @lc code=start
impl Solution {
    pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
        if bloom_day.len() as i32 / k < m {
            return -1;
        }
        let mut low = *bloom_day.iter().min().unwrap();
        let mut high = *bloom_day.iter().max().unwrap();
        let can_make = |days: i32| -> bool {
            let mut flowers = 0;
            let mut bouquets = 0;
            for &b in &bloom_day {
                if b <=days {
                    flowers += 1;
                    if flowers == k {
                        bouquets += 1;
                        flowers = 0;
                    }
                } else {
                    flowers = 0;
                }
            }
            bouquets >= m
        };
        while low < high {
            let mid = ( high+low ) / 2;
            if can_make(mid) {
                high = mid;
            } else {
                low = mid + 1;
            }
        }
        low
    }
}
// @lc code=end

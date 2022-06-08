/*
 * @lc app=leetcode id=2147 lang=rust
 *
 * [2147] Number of Ways to Divide a Long Corridor
 */

// @lc code=start
impl Solution {
    pub fn number_of_ways(corridor: String) -> i32 {
        let mut prev = -1;
        let mut ans = 1;
        let mut cnt = 0;
        for (i, v) in corridor.chars().enumerate() {
            if v == 'P' {
                continue;
            }
            cnt += 1;
            let i = i as i64;
            if cnt > 2 && cnt % 2 > 0 {
                ans = ans * (i - prev) % 1_000_000_007;
            }
            prev = i;
        }
        if cnt<2 || cnt % 2 > 0 {0}else {ans as _}
    }
}
// @lc code=end

/*
 * @lc app=leetcode id=464 lang=rust
 *
 * [464] Can I Win
 */

// @lc code=start
impl Solution {
    pub fn can_i_win(max_choosable_integer: i32, desired_total: i32) -> bool {
        if (1 + max_choosable_integer) * max_choosable_integer / 2 < desired_total {
            return false;
        }
        let sm = 1 << max_choosable_integer;
        let mut dp = vec![false; sm];
        for i in (0..sm).rev() {
            let mut t = desired_total;
            for j in 0..max_choosable_integer {
                if (1 << j) & i != 0 {
                    t -= j + 1;
                }
            }
            for j in 0..max_choosable_integer {
                if (1 << j) & i != 0 {
                    continue;
                }
                if j + 1 >= t || !dp[((1 << j) | i) as usize] {
                    dp[i as usize] = true;
                }
            }
        }
        dp[0]
    }
}
// @lc code=end

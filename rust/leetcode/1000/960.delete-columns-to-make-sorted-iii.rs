/*
 * @lc app=leetcode id=960 lang=rust
 *
 * [960] Delete Columns to Make Sorted III
 */

// @lc code=start
impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        if strs[0].len() < 2 {
            return 0;
        }
        let w = strs[0].len();

        let mut dp = vec![1; w];
        for i in (0..=w - 2).rev() {
            for j in i + 1..w {
                if strs
                    .iter()
                    .all(|row| row.as_bytes()[i] <= row.as_bytes()[j])
                {
                    dp[i] = dp[i].max(1 + dp[j]);
                }
            }
        }
        w as i32 - *dp.iter().max().unwrap()
    }
}
// @lc code=end

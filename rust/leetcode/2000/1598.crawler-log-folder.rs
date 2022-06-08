/*
 * @lc app=leetcode id=1598 lang=rust
 *
 * [1598] Crawler Log Folder
 */

// @lc code=start
impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        let mut ans = 0;
        for s in &logs {
            if *s != "./" && *s != "../" {
                ans += 1;
            } else if *s == "../" {
                if ans > 0 {
                    ans -= 1;
                }
            }
        }
        ans
    }
}
// @lc code=end

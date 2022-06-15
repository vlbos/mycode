/*
 * @lc app=leetcode id=2078 lang=rust
 *
 * [2078] Two Furthest Houses With Different Colors
 */

// @lc code=start
impl Solution {
    pub fn max_distance(colors: Vec<i32>) -> i32 {
        let mut ans = 0;
        for (i, &ci) in colors.iter().enumerate() {
            for (j, &cj) in colors[i + 1..].iter().enumerate() {
                if ci != cj {
                    ans = ans.max(j+1);
                }
            }
        }
        ans as _
    }
}
// @lc code=end

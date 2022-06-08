/*
 * @lc app=leetcode id=1996 lang=rust
 *
 * [1996] The Number of Weak Characters in the Game
 */

// @lc code=start
impl Solution {
    pub fn number_of_weak_characters(properties: Vec<Vec<i32>>) -> i32 {
        let mut properties = properties;
        properties.sort_by(|a, b| {
            if a[0] == b[0] {
                a[1].cmp(&b[1])
            } else {
                b[0].cmp(&a[0])
            }
        });
        let mut max_def = 0;
        let mut ans = 0;
        for p in &properties {
            if p[1] < max_def {
                ans += 1;
            } else {
                max_def = p[1];
            }
        }
        ans
    }
}
// @lc code=end

/*
 * @lc app=leetcode id=1753 lang=rust
 *
 * [1753] Maximum Score From Removing Stones
 */

// @lc code=start
impl Solution {
    pub fn maximum_score(a: i32, b: i32, c: i32) -> i32 {
        let mut v = [a, b, c];
        v.sort();
        let (ab, c) = (v[0] + v[1], v[2]);
        if ab <= c {
            return ab;
        }
        c + (ab - c) / 2
    }
}
// @lc code=end

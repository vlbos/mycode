/*
 * @lc app=leetcode id=455 lang=rust
 *
 * [455] Assign Cookies
 */

// @lc code=start
impl Solution {
    pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
         let mut g = g;
        let mut s = s;
        g.sort();
        s.sort();
        let mut i = 0;
        let mut j = 0;
        let mut c = 0;
        while i < g.len() && j < s.len() {
            while j < s.len() && g[i] > s[j] {
                j += 1;
            }
            if j < s.len() {
                c += 1;
            }
            i += 1;
            j += 1;
        }
        c
    }
}
// @lc code=end


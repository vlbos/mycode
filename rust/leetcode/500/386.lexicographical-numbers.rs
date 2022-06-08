/*
 * @lc app=leetcode id=386 lang=rust
 *
 * [386] Lexicographical Numbers
 */

// @lc code=start
impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut ans = Vec::new();
        let mut nnn = 1;
        while ans.len() < n as usize {
            while nnn <= n {
                ans.push(nnn);
                nnn *= 10;
            }
            while nnn % 10 == 9 || nnn > n {
                nnn /= 10;
            }
            nnn += 1;
        }
        ans
    }
}
// @lc code=end

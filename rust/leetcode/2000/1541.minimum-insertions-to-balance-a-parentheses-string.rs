/*
 * @lc app=leetcode id=1541 lang=rust
 *
 * [1541] Minimum Insertions to Balance a Parentheses String
 */

// @lc code=start
impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let mut ans = 0;
        let mut lc = 0;
        let mut index = 0;
        let bs = s.as_bytes();
        while index < bs.len() {
            let b = bs[index];
            if b == b'(' {
                lc += 1;
                index += 1;
            } else {
                if lc == 0 {
                    ans += 1;
                } else {
                    lc -= 1;
                }
                if index < bs.len() - 1 && bs[index + 1] == b')' {
                    index += 2;
                } else {
                    ans += 1;
                    index += 1;
                }
            }
        }
        ans += lc * 2;
        ans
    }
}
// @lc code=end

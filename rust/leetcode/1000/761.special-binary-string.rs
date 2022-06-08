/*
 * @lc app=leetcode id=761 lang=rust
 *
 * [761] Special Binary String
 */

// @lc code=start
impl Solution {
    pub fn make_largest_special(s: String) -> String {
        let (mut cur, mut last) = (0, 0);
        let mut ans = Vec::new();
        for (i, c) in s.chars().enumerate() {
            cur += if c == '1' { 1 } else { -1 };
            if cur == 0 {
                ans.push(format!(
                    "1{}0",
                    Self::make_largest_special(s[last + 1..i].to_string())
                ));
                last = i + 1;
            }
        }

        ans.sort_by(|a, b| b.cmp(&a));
        ans.concat()
    }
}
// @lc code=end

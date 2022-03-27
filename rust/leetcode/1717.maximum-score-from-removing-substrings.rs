/*
 * @lc app=leetcode id=1717 lang=rust
 *
 * [1717] Maximum Score From Removing Substrings
 */

// @lc code=start
impl Solution {
    pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
        let (mut ca, mut cb) = (0, 0);
        let mut ans = 0;
        for c in s.chars() {
            match c {
                'a' => {
                    if x < y && cb > 0 {
                        cb -= 1;
                        ans += y;
                    } else {
                        ca += 1;
                    }
                }
                'b' => {
                    if x >= y && ca > 0 {
                        ca -= 1;
                        ans += x;
                    } else {
                        cb += 1;
                    }
                }
                _ => {
                    if ca > 0 && cb > 0 {
                        ans += x.min(y) * ca.min(cb);
                    }
                    ca = 0;
                    cb = 0;
                }
            }
        }
        if ca > 0 && cb > 0 {
            ans += x.min(y) * ca.min(cb);
        }
        ans
    }
}
// @lc code=end

/*
 * @lc app=leetcode id=1805 lang=rust
 *
 * [1805] Number of Different Integers in a String
 */

// @lc code=start
impl Solution {
    pub fn num_different_integers(word: String) -> i32 {
        let mut v = Vec::new();
        let mut w = String::new();
        let mut wi = 0;
        for c in word.chars() {
            if c.is_numeric() {
                w.push(c);
            } else {
                if !w.is_empty() {
                    let mut ww = w.clone();
                    ww = ww.trim_start_matches('0').to_string();
                    if !v.contains(&ww) {
                        v.push(ww);
                    }
                }
                w = String::new();
            }
        }
        if !w.is_empty() {
            let mut ww = w.clone();
            ww = ww.trim_start_matches('0').to_string();
            if !v.contains(&ww) {
                v.push(ww);
            }
        }
        v.len() as i32
    }
}
// @lc code=end

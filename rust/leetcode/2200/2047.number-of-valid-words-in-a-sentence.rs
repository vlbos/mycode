/*
 * @lc app=leetcode id=2047 lang=rust
 *
 * [2047] Number of Valid Words in a Sentence
 */

// @lc code=start
impl Solution {
    pub fn count_valid_words(sentence: String) -> i32 {
        let mut ans = 0;
        for s in sentence.split_ascii_whitespace() {
            let mut valid = true;
            let mut hyphens = false;
            for (i, c) in s.chars().enumerate() {
                match c {
                    c if c.is_ascii_digit() => {
                        valid = false;
                        break;
                    }
                    '!' | '.' | ',' => {
                        if i < s.len() - 1 || s.len() > 1 && s.as_bytes()[s.len() - 2] == b'-' {
                            valid = false;
                            break;
                        }
                    }
                    '-' => {
                        if hyphens || i == 0 || i == s.len() - 1 {
                            valid = false;
                            break;
                        }
                        hyphens = true;
                    }
                    _ => {}
                }
            }
            if valid {
                ans += 1;
            }
        }
        ans
    }
}
// @lc code=end

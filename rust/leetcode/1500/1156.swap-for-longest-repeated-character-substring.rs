/*
 * @lc app=leetcode id=1156 lang=rust
 *
 * [1156] Swap For Longest Repeated Character Substring
 */

// @lc code=start
impl Solution {
    pub fn max_rep_opt1(text: String) -> i32 {
        let mut win_count = vec![0; 26];
        let mut char_count = vec![0; 26];
        for b in text.bytes() {
            char_count[(b - b'a') as usize] += 1;
        }
        let (mut start, mut max_char, mut max_count, mut max_win) = (0, 0, 0, 0);
        let mut max_chars = Vec::new();
        for (i, b) in text.bytes().enumerate() {
            let j = (b - b'a') as usize;
            win_count[j] += 1;
            if win_count[j] > max_count {
                max_char = b;
                max_chars = vec![b];
                max_count = win_count[j];
                max_win = max_count + 1;
            } else {
                if win_count[j] == max_count {
                    max_chars.push(b);
                }
                if i - start + 1 > max_win {
                    let k = (text.bytes().nth(start).unwrap() - b'a') as usize;
                    win_count[k] -= 1;
                    start += 1;
                }
            }
        }
        for &b in &max_chars {
            if char_count[(b - b'a') as usize] == max_win {
                max_char = b;
            }
        }
        max_win.min(char_count[(max_char - b'a') as usize]) as _
    }
}
// @lc code=end

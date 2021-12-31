/*
 * @lc app=leetcode id=1160 lang=rust
 *
 * [1160] Find Words That Can Be Formed by Characters
 */

// @lc code=start
impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let mut r = vec![0; 26];
        for c in chars.chars() {
            r[(c as u8 - 'a' as u8) as usize] += 1;
        }
        let mut ans = 0;
        for w in &words {
            let mut flag = true;
            let mut rr = vec![0; 26];
            for c in w.chars() {
                let i = (c as u8 - 'a' as u8) as usize;
                rr[i] += 1;
                if rr[i] > r[i] {
                    flag = false;
                    break;
                }
            }
            if flag {
                ans += w.len();
            }
        }
        ans as i32
    }
}
// @lc code=end

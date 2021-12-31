/*
 * @lc app=leetcode id=1002 lang=rust
 *
 * [1002] Find Common Characters
 */

// @lc code=start
impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        let mut r = Vec::<String>::new();
        let mut m = vec![i32::MAX; 26];
        let mut f = vec![0; 26];
        for w in &words {
            f = vec![0; 26];
            for c in w.chars() {
                let index = (c as u8 - 'a' as u8) as usize;
                f[index] += 1;
            }
            for i in 0..26 {
                m[i] = m[i].min(f[i]);
            }
        }
        for i in 0..26 {
            if m[i] > 0 {
                r.extend(vec![
                    (('a' as u8 + i as u8) as char).to_string();
                    m[i] as usize
                ]);
            }
        }
        r
    }
}
// @lc code=end

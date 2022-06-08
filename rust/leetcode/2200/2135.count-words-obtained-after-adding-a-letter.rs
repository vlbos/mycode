/*
 * @lc app=leetcode id=2135 lang=rust
 *
 * [2135] Count Words Obtained After Adding a Letter
 */

// @lc code=start
impl Solution {
    pub fn word_count(start_words: Vec<String>, target_words: Vec<String>) -> i32 {
        let mask = |w: &String| -> i32 {
            let mut ans = 0;
            for b in w.bytes() {
                ans |= 1 << (b - b'a') as usize;
            }
            ans
        };
        let mut masks = std::collections::HashSet::new();
        for s in &start_words {
            let m = mask(s);
            for i in 0..26 {
                if m & (1 << i) == 0 {
                    masks.insert(m | (1 << i));
                }
            }
        }
        let mut ans = 0;
        for s in &target_words {
            let m = mask(s);
            if masks.contains(&m) {
                ans += 1;
            }
        }
        ans
    }
}
// @lc code=end

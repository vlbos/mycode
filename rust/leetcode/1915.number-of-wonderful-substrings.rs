/*
 * @lc app=leetcode id=1915 lang=rust
 *
 * [1915] Number of Wonderful Substrings
 */

// @lc code=start
impl Solution {
    pub fn wonderful_substrings(word: String) -> i64 {
        let mut m = std::collections::HashMap::new();
        m.insert(0, 1);
        let mut ans = 0;
        let mut mask = 0;
        for b in word.bytes() {
            let i = (b - b'a') as usize;
            mask ^= (1 << i) as i64;
            if let Some(&v) = m.get(&mask) {
                ans += v;
            }
            for i in 0..10 {
                let mask_pre = mask ^ (1 << i) as i64;
                if let Some(&v) = m.get(&mask_pre) {
                    ans += v;
                }
            }
            *m.entry(mask).or_insert(0) += 1;
        }
        ans
    }
}
// @lc code=end

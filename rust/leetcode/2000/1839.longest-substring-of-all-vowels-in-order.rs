/*
 * @lc app=leetcode id=1839 lang=rust
 *
 * [1839] Longest Substring Of All Vowels in Order
 */

// @lc code=start
impl Solution {
    pub fn longest_beautiful_substring(word: String) -> i32 {
        if word.len() < 5 {
            return 0;
        }
        let v = [b'a', b'e', b'i', b'o', b'u'];
        let (mut i, mut j, mut k) = (0, 0, 0);
        let w = word.as_bytes();
        let mut ans = 0;
        while j < w.len() {
            if w[j] == v[k] {
                j += 1;
                i += 1;
                continue;
            }
            if k == v.len() - 1 {
                ans = ans.max(i);
                k = 0;
                i = 0;
                continue;
            }
            if i > 0 && w[j] == v[k + 1] {
                k += 1;
                continue;
            }
            if w[j] == v[0] {
                k = 0;
                i = 0;
                continue;
            }
            while j < w.len() && w[j] != v[0] {
                j += 1;
                i = 0;
            }
        }
        if k == v.len() - 1 {
            ans = ans.max(i);
        }
        ans
    }
}
// @lc code=end

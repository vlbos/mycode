/*
 * @lc app=leetcode id=1525 lang=rust
 *
 * [1525] Number of Good Ways to Split a String
 */

// @lc code=start
impl Solution {
    pub fn num_splits(s: String) -> i32 {
         let mut r = vec![0; 26];
        for b in s.bytes() {
            r[(b - b'a') as usize] += 1;
        }
        let mut l = vec![0; 26];
        let mut ans = 0;
        for b in s.bytes() {
            l[(b - b'a') as usize] += 1;
            r[(b - b'a') as usize] -= 1;
            if l.iter().filter(|x|(**x)>0).count() == r.iter().filter(|x|(**x)>0).count() {
                ans += 1;
            }
        }
        ans
    }
}
// @lc code=end

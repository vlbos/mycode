/*
 * @lc app=leetcode id=467 lang=rust
 *
 * [467] Unique Substrings in Wraparound String
 */

// @lc code=start
impl Solution {
    pub fn find_substring_in_wrapround_string(p: String) -> i32 {
        let is_continous = |prev: u8, curr: u8| {
            if prev == b'z' {
                return curr == b'a';
            }
            prev + 1 == curr
        };
        let pp = p.bytes().collect::<Vec<u8>>();
        let mut dp = vec![0; 26];
        let mut l = 0;
        for (i, b) in pp.iter().enumerate() {
            if i > 0 && is_continous(pp[i - 1], *b) {
                l += 1;
            } else {
                l = 1;
            }
            dp[(*b - b'a') as usize] = dp[(*b - b'a') as usize].max(l);
        }
        dp.iter().sum::<i32>()
    }
}
// @lc code=end

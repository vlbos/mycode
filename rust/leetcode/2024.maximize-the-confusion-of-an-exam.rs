/*
 * @lc app=leetcode id=2024 lang=rust
 *
 * [2024] Maximize the Confusion of an Exam
 */

// @lc code=start
impl Solution {
    pub fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
        let n = answer_key.len();
        let b = answer_key.as_bytes();
        let (mut l, mut r) = (0, 0);
        let (mut ct, mut cf) = (0, 0);
        let mut ans = 0;
        while r < n {
            if b[r] == b'T' {
                ct += 1;
            } else {
                cf += 1;
            }
            while ct > k && cf > k {
                if b[l] == b'T' {
                    ct -= 1;
                } else {
                    cf -= 1;
                }
                l += 1;
            }
            ans = ans.max(r - l + 1);
            r += 1;
        }
        ans  as _
    }
}
// @lc code=end

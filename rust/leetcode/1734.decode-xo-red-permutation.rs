/*
 * @lc app=leetcode id=1734 lang=rust
 *
 * [1734] Decode XORed Permutation
 */

// @lc code=start
impl Solution {
    pub fn decode(encoded: Vec<i32>) -> Vec<i32> {
        let n = encoded.len() + 1;
        let t = (1..=n).reduce(|a, x| a ^ x).unwrap() as i32;
        let mut o = 0;
        for i in (1..n).step_by(2) {
            o ^= encoded[i];
        }
        let mut ans = vec![0; n];
        ans[0] = t ^ o;
        for i in (1..n) {
            ans[i] = ans[i - 1] ^ encoded[i - 1];
        }
        ans
    }
}
// @lc code=end

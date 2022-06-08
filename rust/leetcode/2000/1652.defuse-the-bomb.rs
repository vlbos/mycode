/*
 * @lc app=leetcode id=1652 lang=rust
 *
 * [1652] Defuse the Bomb
 */

// @lc code=start
impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        let mut dec = vec![0; code.len()];
        if k == 0 {
            return dec;
        }
        let d = if k > 0 { 1 } else { 0 };
        let k = if k < 0 { k.abs() } else { k } as usize;
        let mut codep = code.clone();
        if d > 0 {
            codep.extend(&code[..k]);
        } else {
            codep = (&code[code.len() - k..]).to_vec();
            codep.extend(code);
        }

        let mut presum = vec![0; codep.len() + 1];

        for i in 1..=codep.len() {
            presum[i] = presum[i - 1] + codep[i - 1];
        }
        for i in 0..dec.len() {
            dec[i] = presum[i + k + d] - presum[i + d];
        }

        dec
    }
}
// @lc code=end

/*
 * @lc app=leetcode id=60 lang=rust
 *
 * [60] Permutation Sequence
 */

// @lc code=start
impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
         let mut k = k;
        let n = n as usize;
        let mut factorial = vec![0; n];
        factorial[0]=1;
        for i in 1..n {
            factorial[i] = factorial[i - 1] * i as i32;
        }
        k -= 1;
        let mut ans = String::new();
        let mut valid = vec![1; n + 1];
        for i in 1..=n {
            let mut order = k / factorial[n - i] + 1;
            for j in 1..=n {
                order -= valid[j];
                if order == 0 {
                    ans.push((b'0' + j as u8) as char);
                    valid[j] = 0;
                    break;
                }
            }
            k %= factorial[n - i];
        }
        ans
    }
}
// @lc code=end

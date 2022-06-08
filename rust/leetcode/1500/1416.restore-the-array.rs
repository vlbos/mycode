/*
 * @lc app=leetcode id=1416 lang=rust
 *
 * [1416] Restore The Array
 */

// @lc code=start
impl Solution {
    pub fn number_of_arrays(s: String, k: i32) -> i32 {
        let n = s.len();
        let k = k as i64;
        let bs = s.as_bytes();
        let mut f = vec![0; n+1];
        f[0] = 1;
        for i in 1..=n {
            let mut base = 1;
            let mut num = 0;
            for j in (0..i).rev() {
                if i - j > 10 {
                    break;
                }
                num += base * (bs[j] - b'0') as i64;
                if num > k {
                    break;
                }
                if bs[j] != b'0' {
                    f[i] += f[j];
                    f[i] %= 1_000_000_007;
                }
                base *= 10;
            }
        }
        f[n] as _
    }
}
// @lc code=end

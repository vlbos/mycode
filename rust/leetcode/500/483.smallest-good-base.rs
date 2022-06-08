/*
 * @lc app=leetcode id=483 lang=rust
 *
 * [483] Smallest Good Base
 */

// @lc code=start
impl Solution {
    pub fn smallest_good_base(n: String) -> String {
        let val = n.parse::<i64>().unwrap();
        let max = ((val as f64).log10() / 2f64.log10()).floor() as i64;
        for m in (2..=max).rev() {
            let k = (val as f64).powf(1.0 / m as f64) as i64;
            let mut mul = 1;
            let mut sum = 1;
            for i in 0..m {
                mul *= k;
                sum += mul;
            }
            if sum == val {
                return k.to_string();
            }
        }
        (val - 1).to_string()
    }
}
// @lc code=end

/*
 * @lc app=leetcode id=50 lang=rust
 *
 * [50] Pow(x, n)
 */

// @lc code=start
impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        fn my_pow_64(x: f64, n: i64) -> f64 {
            if n == 0 {
                return 1f64;
            }

            let r = my_pow_64(x, n / 2);
            if n %2== 0 {
                r * r
            } else {
                r * r * x
            }
        }
        let mut n = n as i64;
        let y = my_pow_64(x, if n < 0 {-n}else{n});
        if n < 0 {
            1f64 / y
        } else {
            y
        }
    }
}
// @lc code=end

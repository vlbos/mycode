/*
 * @lc app=leetcode id=1492 lang=rust
 *
 * [1492] The kth Factor of n
 */

// @lc code=start
impl Solution {
    pub fn kth_factor(n: i32, k: i32) -> i32 {
        let mut count = 0;
        let mut f = 1;
        while f * f <= n {
            if n % f == 0 {
                count += 1;
                if count == k {
                    return f;
                }
            }
            f += 1;
        }
        f -= 1;
        if f * f == n {
            f -= 1;
        }
        while f > 0 {
            if n % f == 0 {
                count += 1;
                if count == k {
                    return n / f;
                }
            }
            f -= 1;
        }
        -1
    }
}
// @lc code=end

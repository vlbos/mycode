/*
 * @lc app=leetcode id=1411 lang=rust
 *
 * [1411] Number of Ways to Paint N × 3 Grid
 */

// @lc code=start
impl Solution {
    pub fn num_of_ways(n: i32) -> i32 {
        let (mut fi0, mut fi1) = (6i64, 6i64);
        let p = 1_000_000_007;
        for _ in 2..=n {
            let (mut new_fi0, mut new_fi1) = ((fi0 * 2 + fi1 * 2) % p, (fi0 * 2 + fi1 * 3) % p);
            fi0 = new_fi0;
            fi1 = new_fi1;
        }
        ((fi0 + fi1) % p) as _
    }
}
// @lc code=end

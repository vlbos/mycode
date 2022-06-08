/*
 * @lc app=leetcode id=1573 lang=rust
 *
 * [1573] Number of Ways to Split a String
 */

// @lc code=start
impl Solution {
    pub fn num_ways(s: String) -> i32 {
        let p = 1000_000_007;
        let n = s.len();
        let ones = s
            .chars()
            .enumerate()
            .filter(|x| x.1 == '1')
            .map(|x| x.0)
            .collect::<Vec<usize>>();
        let m = ones.len();
        if m % 3 > 0 {
            return 0;
        }
        if m == 0 {
            let nn = n as i64;
            return (((nn - 1) * (nn - 2) / 2) % p) as _;
        }
        let (i1, i2) = (m / 3, m / 3 * 2);
        let c1 = (ones[i1] - ones[i1 - 1]) as i64;
        let c2 = (ones[i2] - ones[i2 - 1]) as i64;
        ((c1 * c2) % p) as _
    }
}
// @lc code=end

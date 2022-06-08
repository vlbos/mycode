/*
 * @lc app=leetcode id=1420 lang=rust
 *
 * [1420] Build Array Where You Can Find The Maximum Exactly K Comparisons
 */

// @lc code=start
impl Solution {
    pub fn num_of_arrays(n: i32, m: i32, k: i32) -> i32 {
        if k == 0 {
            return 0;
        }
        let (m, n, k) = (m as usize, n as usize, k as usize);
        let mut f = vec![vec![vec![0; m + 1]; k + 1]; n + 1];
        for j in 1..=m {
            f[1][1][j] = 1;
        }
        let p = 1_000_000_007;
        for i in 2..=n {
            for s in 1..=i.min(k) {
                let mut presum_j = 0;
                for j in 1..=m {
                    f[i][s][j] = ((f[i - 1][s][j] * j as i64) % p + presum_j) % p;
                    presum_j += f[i - 1][s - 1][j];
                    presum_j %= p;
                }
            }
        }
        f.remove(n).remove(k).into_iter().reduce(|acc, v| (acc + v) % p).unwrap() as _
    }
}
// @lc code=end

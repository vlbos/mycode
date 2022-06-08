/*
 * @lc app=leetcode id=458 lang=rust
 *
 * [458] Poor Pigs
 */

// @lc code=start
impl Solution {
    pub fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
        // let states =minutes_to_test /minutes_to_die +1;
        // ( (buckets as f64).log2()/(states as f64).log2()).ceil() as _
       if buckets == 1 {
            return 0;
        }
        let (m, n) = (
            buckets as usize,
            (minutes_to_test / minutes_to_die) as usize,
        );
        let mut combinations = vec![vec![0; m + 1]; m + 1];
        combinations[0][0] = 1;
        let mut f = vec![vec![0; n + 1]; m];
        for i in 0..m {
            f[i][0] = 1;
        }
        for j in 0..=n {
            f[0][j] = 1;
        }

        for i in 1..m {
            combinations[i][0] = 1;
            combinations[i][i] = 1;

            for j in 1..i {
                combinations[i][j] = combinations[i - 1][j - 1] + combinations[i - 1][j];
            }
            for j in 1..=n {
                for k in 0..=i {
                    f[i][j] += f[k][j - 1] * combinations[i][i - k];
                }
            }
            if f[i][n] >= buckets{
                return i as _;
            }
        }
        0
    }
}
// @lc code=end

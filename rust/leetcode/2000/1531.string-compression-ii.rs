/*
 * @lc app=leetcode id=1531 lang=rust
 *
 * [1531] String Compression II
 */

// @lc code=start
impl Solution {
    pub fn get_length_of_optimal_compression(s: String, k: i32) -> i32 {
        let calc = |x: i32| match x {
            1 => 1,
            _ if x < 10 => 2,
            _ if x < 100 => 3,
            _ => 4,
        };
        let n = s.len();
        let k = k as usize;
        let mut f = vec![vec![i32::MAX / 2; k + 1]; n + 1];
        f[0][0] = 0;
        let bs = s.as_bytes();
        for i in 1..=n {
            for j in 0..=i.min(k) {
                if j > 0 {
                    f[i][j] = f[i - 1][j - 1];
                }
                let (mut same, mut diff) = (0, 0);
                for i0 in (1..=i).rev() {
                    if diff > j {
                        break;
                    }
                    if bs[i0 - 1] == bs[i - 1] {
                        same += 1;
                        f[i][j] = f[i][j].min(f[i0 - 1][j - diff] + calc(same));
                    } else {
                        diff += 1;
                    }
                }
            }
        }
        f[n][k]
    }
}
// @lc code=end

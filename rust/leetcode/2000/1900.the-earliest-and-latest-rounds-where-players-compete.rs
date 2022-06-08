/*
 * @lc app=leetcode id=1900 lang=rust
 *
 * [1900] The Earliest and Latest Rounds Where Players Compete
 */

// @lc code=start
impl Solution {
    pub fn earliest_and_latest(n: i32, first_player: i32, second_player: i32) -> Vec<i32> {
        fn dp(
            n: usize,
            f: usize,
            s: usize,
            ff: &mut Vec<Vec<Vec<i32>>>,
            g: &mut Vec<Vec<Vec<i32>>>,
        ) -> Vec<i32> {
            if ff[n][f][s] > 0 {
                return vec![ff[n][f][s], g[n][f][s]];
            }
            if f + s == n + 1 {
                return vec![1, 1];
            }
            if f + s > n + 1 {
                return dp(n, n + 1 - s, n + 1 - f, ff, g);
            }
            let (mut earliest, mut latest) = (i32::MAX, i32::MIN);
            let n_half = (n + 1) / 2;
            if s <= n_half {
                for i in 0..f {
                    for j in 0..s - f {
                        let d = dp(n_half, i + 1, i + j + 2, ff, g);
                        earliest = earliest.min(d[0]);
                        latest = latest.max(d[1]);
                    }
                }
            } else {
                let s_prime = n + 1 - s;
                let mid = (n - 2 * s_prime + 1) / 2;
                for i in 0..f {
                    for j in 0..s_prime - f {
                        let d = dp(n_half, i + 1, i + j + mid + 2, ff, g);
                        earliest = earliest.min(d[0]);
                        latest = latest.max(d[1]);
                    }
                }
            }
            ff[n][f][s] = earliest + 1;
            g[n][f][s] = latest + 1;
            vec![earliest + 1, latest + 1]
        }
        let (mut ff, mut g) = (
            vec![vec![vec![0; 30]; 30]; 30],
            vec![vec![vec![0; 30]; 30]; 30],
        );
        let (first_player, second_player) = if first_player > second_player {
            (second_player as usize, first_player as usize)
        } else {
            (first_player as usize, second_player as usize)
        };
        dp(n as usize, first_player, second_player, &mut ff, &mut g)
    }
}
// @lc code=end

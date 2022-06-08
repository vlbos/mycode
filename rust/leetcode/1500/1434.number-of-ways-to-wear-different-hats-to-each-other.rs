/*
 * @lc app=leetcode id=1434 lang=rust
 *
 * [1434] Number of Ways to Wear Different Hats to Each Other
 */

// @lc code=start
impl Solution {
    pub fn number_ways(hats: Vec<Vec<i32>>) -> i32 {
        let n = hats.len();
        let max_hat_id = hats
            .iter().map(|x| *x.iter().max().unwrap())
            .max()
            .unwrap() as usize;
        let mut hat_to_person = std::collections::HashMap::new();
        for (i, hat) in hats.iter().enumerate() {
            for &h in hat {
                hat_to_person
                    .entry(h as usize)
                    .or_insert(Vec::new())
                    .push(i);
            }
        }
        let mut f = vec![vec![0; (1 << n)]; max_hat_id + 1];
        f[0][0] = 1;
        for i in 1..=max_hat_id {
            for mask in 0..(1 << n) {
                f[i][mask] = f[i - 1][mask] as i64;
                for j in hat_to_person.get(&i).unwrap_or(&Vec::new()) {
                    if mask & (1 << j) > 0 {
                        f[i][mask] += f[i - 1][mask ^ (1 << j)];
                    }
                }
                f[i][mask] %= 1_000_000_007;
            }
        }
        f[max_hat_id][(1 << n) - 1] as _
    }
}
// @lc code=end

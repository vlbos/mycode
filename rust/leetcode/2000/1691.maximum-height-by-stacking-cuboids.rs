/*
 * @lc app=leetcode id=1691 lang=rust
 *
 * [1691] Maximum Height by Stacking Cuboids
 */

// @lc code=start
impl Solution {
    pub fn max_height(cuboids: Vec<Vec<i32>>) -> i32 {
        let n = cuboids.len();
        let mut cuboids: Vec<Vec<i32>> = cuboids
            .into_iter()
            .map(|mut x| {
                x.sort();
                x
            })
            .collect();
        cuboids.sort_by_key(|x| (x[2], x[0] + x[1]));
        let mut f = vec![0; n];
        for i in 0..n {
            for j in 0..i {
                if cuboids[j][0] <= cuboids[i][0]
                    && cuboids[j][1] <= cuboids[i][1]
                    && cuboids[j][2] <= cuboids[i][2]
                {
                    f[i] = f[i].max(f[j]);
                }
            }
            f[i] += cuboids[i][2];
        }
        f.into_iter().max().unwrap()
    }
}
// @lc code=end

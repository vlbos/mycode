/*
 * @lc app=leetcode id=1334 lang=rust
 *
 * [1334] Find the City With the Smallest Number of Neighbors at a Threshold Distance
 */

// @lc code=start
impl Solution {
    pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
        let n = n as usize;
        let mut dis = vec![vec![i32::MAX / 2; n]; n];
        for i in 0..n {
            dis[i][i] = 0;
        }
        for e in &edges {
            let (i, j) = (e[0] as usize, e[1] as usize);
            dis[i][j] = e[2];
            dis[j][i] = e[2];
        }
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    dis[i][j] = dis[i][j].min(dis[i][k] + dis[k][j]);
                }
            }
        }
        let mut ans = 0;
        let mut count = n + 1;
        for i in 0..n {
            let mut cur = 0;
            for j in 0..n {
                if dis[i][j] <= distance_threshold {
                    cur += 1;
                }
            }
            if cur <= count {
                ans = i as i32;
                count = cur;
            }
        }
        ans
    }
}
// @lc code=end

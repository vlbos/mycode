/*
 * @lc app=leetcode id=1765 lang=rust
 *
 * [1765] Map of Highest Peak
 */

// @lc code=start
impl Solution {
    pub fn highest_peak(is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n) = (is_water.len() as i32, is_water[0].len() as i32);
        let mut ans = is_water
            .iter()
            .map(|x| x.iter().map(|y| *y - 1).collect::<Vec<i32>>())
            .collect::<Vec<Vec<i32>>>();
        let mut q = std::collections::VecDeque::new();
        for (i, r) in is_water.iter().enumerate() {
            for (j, &c) in r.iter().enumerate() {
                if c > 0 {
                    q.push_back((i, j));
                }
            }
        }
        let d = [0, 1, 0, -1, 0];
        while let Some((i, j)) = q.pop_front() {
            for k in 0..d.len() - 1 {
                let (x, y) = (i as i32 + d[k], j as i32 + d[k + 1]);
                if x >= 0 && x < m && y >= 0 && y < n && ans[x as usize][y as usize] == -1 {
                    ans[x as usize][y as usize] = ans[i][j] + 1;
                    q.push_back((x as usize, y as usize));
                }
            }
        }
        ans
    }
}
// @lc code=end

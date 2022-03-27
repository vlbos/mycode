/*
 * @lc app=leetcode id=1878 lang=rust
 *
 * [1878] Get Biggest Three Rhombus Sums in a Grid
 */

// @lc code=start
impl Solution {
    pub fn get_biggest_three(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let (m, n) = (grid.len(),grid[0].len());
        let (mut sum1, mut sum2) = (vec![vec![0; n + 2]; m + 1], vec![vec![0; n + 2]; m + 1]);
        for i in 1..m + 1 {
            for j in 1..n + 1 {
                sum1[i][j] = sum1[i - 1][j - 1] + grid[i - 1][j - 1];
                sum2[i][j] = sum2[i - 1][j + 1] + grid[i - 1][j - 1];
            }
        }
        let mut ans = vec![0; 3];
        let put = |x: i32, ans: &mut Vec<i32>| {
            if x > ans[0] {
                ans.insert(0, x);
                ans.pop();
            } else if x != ans[0] && x > ans[1] {
                ans.insert(1, x);
                ans.pop();
            } else if x != ans[0] && x != ans[1] && x > ans[2] {
                ans[2] = x;
            }
        };
        for i in 0..m {
            for j in 0..n {
                put(grid[i][j], &mut ans);
                for k in (i + 2..m).step_by(2) {
                    let (ux, uy) = (i, j);
                    let (dx, dy) = (k, j);
                    if k < i || j < (k - i) / 2 || j + (k - i) / 2 >= n {
                        break;
                    }
                    let (lx, ly) = ((i + k) / 2, j - (k - i) / 2);
                    let (rx, ry) = ((i + k) / 2, j + (k - i) / 2);
                    put(
                        (sum2[lx + 1][ly + 1] - sum2[ux][uy + 2])
                            + (sum1[rx + 1][ry + 1] - sum1[ux][uy])
                            + (sum1[dx + 1][dy + 1] - sum1[lx][ly])
                            + (sum2[dx + 1][dy + 1] - sum2[rx][ry + 2])
                            - (grid[ux][uy] + grid[dx][dy] + grid[lx][ly] + grid[rx][ry]),
                        &mut ans,
                    );
                }
            }
        }
        ans.iter().filter(|&x| *x != 0).cloned().collect::<Vec<i32>>()
    }
}
// @lc code=end

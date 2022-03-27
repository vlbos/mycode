/*
 * @lc app=leetcode id=1162 lang=rust
 *
 * [1162] As Far from Land as Possible
 */

// @lc code=start
impl Solution {
    pub fn max_distance(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let n = grid.len();
        let land_count = grid
            .iter()
            .map(|x| x.iter().filter(|&a| *a == 1).count())
            .sum::<usize>();
        let mut turn_count = 0;
        if land_count == n * n || land_count == 0 {
            return -1;
        }
        let mut last_space_count = n * n - land_count;
        while last_space_count != 0 {
            turn_count += 1;
            for i in 0..n {
                for j in 0..n {
                    if grid[i][j] != turn_count {
                        continue;
                    }
                    if i > 0 && grid[i - 1][j] == 0 {
                        grid[i - 1][j] = turn_count + 1;
                        last_space_count -= 1;
                    }
                    if i < n - 1 && grid[i + 1][j] == 0 {
                        grid[i + 1][j] = turn_count + 1;
                        last_space_count -= 1;
                    }
                    if j > 0 && grid[i][j - 1] == 0 {
                        grid[i][j - 1] = turn_count + 1;
                        last_space_count -= 1;
                    }
                    if j < n - 1 && grid[i][j + 1] == 0 {
                        grid[i][j + 1] = turn_count + 1;
                        last_space_count -= 1;
                    }
                }
            }
        }
        turn_count
    }
}
// @lc code=end

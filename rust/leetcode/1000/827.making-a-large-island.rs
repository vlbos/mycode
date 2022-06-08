/*
 * @lc app=leetcode id=827 lang=rust
 *
 * [827] Making A Large Island
 */

// @lc code=start
impl Solution {
    pub fn largest_island(grid: Vec<Vec<i32>>) -> i32 {
       let n = grid.len();
        let mut grid = grid;
        let mut area = std::collections::HashMap::new();
        let mut index = 2;
        for r in 0..n {
            for c in 0..n {
                if grid[r][c] == 1 {
                    area.insert(index, dfs(r, c,index, &mut grid));
                    index += 1;
                }
            }
        }
        fn dfs(r: usize, c: usize, index: i32, grid: &mut Vec<Vec<i32>>) -> i32 {
            let n = grid.len();
            let mut ans = 1;
            grid[r][c] = index;
            for (nr, nc) in neighbors(r as i32, c as i32, n as i32) {
                if grid[nr][nc] == 1 {
                    ans += dfs(nr, nc, index, grid);
                }
            }
            ans
        }
        fn neighbors(r: i32, c: i32, n: i32)->Vec<(usize,usize)> {
            let dirs = [0, 1, 0, -1, 0];
            let mut ans = Vec::new();
            for i in 0..dirs.len() - 1 {
                let (nr, nc) = (r + dirs[i], c + dirs[i + 1]);
                if nr < 0 || nr >= n || nc < 0 || nc >= n {
                    continue;
                }
                ans.push((nr as usize, nc as usize));
            }
            ans
        }
        let mut ans = *area.values().max().unwrap_or(&0);
        for r in 0..n {
            for c in 0..n {
                if grid[r][c] == 0 {
                    let mut seen = std::collections::HashSet::new();

                    for (nr, nc) in neighbors(r as i32, c as i32, n as i32) {
                        if grid[nr][nc] > 1 {
                            seen.insert(grid[nr][nc]);
                        }
                    }
                    let bns = seen.into_iter().map(|x| area[&x]).sum::<i32>() + 1;
                    ans = ans.max(bns);
                }
            }
        }
        ans
    }
}
// @lc code=end

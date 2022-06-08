/*
 * @lc app=leetcode id=934 lang=rust
 *
 * [934] Shortest Bridge
 */

// @lc code=start
impl Solution {
    pub fn shortest_bridge(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();
        let neighbours = |i: usize, j: usize| -> Vec<(usize, usize)> {
            let mut ans = Vec::new();
            if i > 0 {
                ans.push((i - 1, j));
            }
            if j > 0 {
                ans.push((i, j - 1));
            }
            if i + 1 < rows {
                ans.push((i + 1, j));
            }
            if j + 1 < cols {
                ans.push((i, j + 1));
            }

            ans
        };
        let get_components = || -> Vec<Vec<i32>> {
            let mut colors = vec![vec![0; cols]; rows];
            let mut t = 0;
            for i in 0..rows {
                for j in 0..cols {
                    if colors[i][j] != 0 || grid[i][j] != 1 {
                        continue;
                    }

                    let mut stack = Vec::new();
                    t += 1;
                    colors[i][j] = t;
                    stack.push((i, j));
                    while let Some((r0, c0)) = stack.pop() {
                        for (r, c) in neighbours(r0, c0) {
                            if colors[r][c] != 0 || grid[r][c] != 1 {
                                continue;
                            }
                            colors[r][c] = t;
                            stack.push((r, c));
                        }
                    }
                }
            }
            colors
        };
        let mut target = std::collections::HashSet::new();
        let mut q = std::collections::VecDeque::new();
        let mut colors = get_components();
        for i in 0..rows {
            for j in 0..cols {
                if colors[i][j] == 1 {
                    q.push_back((i, j, 0));
                } else if colors[i][j] == 2 {
                    target.insert((i, j));
                }
            }
        }
        while let Some((r0, c0, d)) = q.pop_front() {
            if target.contains(&(r0, c0)) {
                return d - 1;
            }
            for (r, c) in neighbours(r0, c0) {
                if colors[r][c] != 1 {
                    colors[r][c] = 1;
                    q.push_back((r, c, d + 1));
                }
            }
        }
        -1
    }
}
// @lc code=end

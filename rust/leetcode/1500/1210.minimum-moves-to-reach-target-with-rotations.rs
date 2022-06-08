/*
 * @lc app=leetcode id=1210 lang=rust
 *
 * [1210] Minimum Moves to Reach Target with Rotations
 */

// @lc code=start
impl Solution {
    pub fn minimum_moves(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        use std::collections::HashSet;
        use std::collections::VecDeque;
        let mut visited = HashSet::new();
        let mut q = VecDeque::new();
        q.push_back((0, 0, 0, 1, 0));
        let check = |x0: usize,
                     y0: usize,
                     x1: usize,
                     y1: usize,
                     step: i32,
                     visited: &mut HashSet<(usize, usize, usize, usize)>,
                     q: &mut VecDeque<(usize, usize, usize, usize, i32)>|
         -> bool {
            if x0 == n - 1 && y0 == n - 2 && x1 == n - 1 && y1 == n - 1 {
                return true;
            }
            if visited.contains(&(x0, y0, x1, y1)) {
                return false;
            }
            visited.insert((x0, y0, x1, y1));
            q.push_back((x0, y0, x1, y1, step + 1));
            false
        };
        while let Some((x0, y0, x1, y1, step)) = q.pop_front() {
            if y0 + 1 < n && grid[x0][y0 + 1] == 0 && y1 + 1 < n && grid[x1][y1 + 1] == 0 {
                if check(x0, y0 + 1, x1, y1 + 1, step, &mut visited, &mut q) {
                    return step + 1;
                }
                if y0 == y1 {
                    if check(x0, y0, x0, y0 + 1, step, &mut visited, &mut q) {
                        return step + 1;
                    }
                }
            }
            if x0 + 1 < n && grid[x0 + 1][y0] == 0 && x1 + 1 < n && grid[x1 + 1][y1] == 0 {
                if check(x0 + 1, y0, x1 + 1, y1, step, &mut visited, &mut q) {
                    return step + 1;
                }
                if x0 == x1 {
                    if check(x0, y0, x0 + 1, y0, step, &mut visited, &mut q) {
                        return step + 1;
                    }
                }
            }
        }
        -1
    }
}
// @lc code=end

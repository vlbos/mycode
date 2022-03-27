/*
 * @lc app=leetcode id=1926 lang=rust
 *
 * [1926] Nearest Exit from Entrance in Maze
 */

// @lc code=start
impl Solution {
    pub fn nearest_exit(maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
        let (m, n) = (maze.len() as i32, maze[0].len() as i32);
        let mut q = std::collections::VecDeque::new();
        let d = [0, 1, 0, -1, 0];
        let mut maze = maze;
        let mut e = entrance;
        e.push(0);
        q.push_back(e.clone());
        maze[e[0] as usize][e[1] as usize] = '+';
        while let Some(e) = q.pop_front() {
            for k in 0..d.len() - 1 {
                let (nx, ny) = (e[0] + d[k], e[1] + d[k + 1]);
                if nx < 0 || nx >= m || ny < 0 || ny >= n || maze[nx as usize][ny as usize] == '+' {
                    continue;
                }
                if nx == 0 || nx == m - 1 || ny == 0 || ny == n - 1 {
                    return e[2] + 1;
                }
                maze[nx as usize][ny as usize] = '+';
                q.push_back(vec![nx, ny, e[2] + 1]);
            }
        }
        -1
    }
}
// @lc code=end

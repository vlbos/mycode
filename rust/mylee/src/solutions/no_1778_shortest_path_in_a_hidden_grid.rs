// 1778\. Shortest Path in a Hidden Grid
// =====================================

// This is an **interactive problem**.

// You are given a robot in a hidden grid, and it wants to go to a target cell in this grid.
// The grid is of size `m x n`, and each cell in the grid can be empty or blocked.
// It is **guaranteed** that the start point and the robot's destination are different, and neither of them is blocked.

// You want to find the robot's minimum distance to the target cell.
// However, you do not know the grid's dimensions, or the starting point of the robot, or its target destination.
//  You are only allowed to ask queries to your `GridMaster` object.

// You are given a class `GridMaster` which you can call the following functions from:

// *   `boolean GridMaster.canMove(char direction)` returns `true` if the robot can move in that direction. Otherwise, it returns `false`.
// *   `void GridMaster.move(char direction)` moves the robot in that direction.
// If this move would move the robot to a blocked cell or off the grid, it will be **ignored,** and the robot would remain in the same position.
// *   `boolean GridMaster.isTarget()` returns `true` if the robot is currently on the target cell. Otherwise, it returns `false`.

// Note that `direction` in the above functions should be a character from `{'U','D','L','R'}`, representing the directions up, down, left, and right, respectively.

// Return _the minimum distance between the robot's initial starting cell and the target cell if there is a path between them_. Otherwise, return `-1`.

// **Custom testing:**

// The test input is read as a 2D matrix `grid` of size `m x n` where:

// *   `grid[i][j] == -1` indicates that the robot is in cell `(i, j)`.
// *   `grid[i][j] == 0` indicates that the cell `(i, j)` is blocked.
// *   `grid[i][j] == 1` indicates that the cell `(i, j)` is empty.
// *   `grid[i][j] == 2` indicates that the cell `(i, j)` is the target cell.

// There is exactly one `-1` and `2` in `grid`. Remember that you will not have this information in your code.

// **Example 1:**

// **Input:** grid = \[\[1,2\],\[-1,0\]\]
// **Output:** 2
// **Explanation:** One possible interaction is described below:
// The robot is initially standing on cell (1, 0), denoted by the -1.
// - master.canMove('U') returns True.
// - master.canMove('D') returns False.
// - master.canMove('L') returns False.
// - master.canMove('R') returns False.
// - master.move('U') moves the robot to the cell (0, 0).
// - master.isTarget() returns False.
// - master.canMove('U') returns False.
// - master.canMove('D') returns True.
// - master.canMove('L') returns False.
// - master.canMove('R') returns True.
// - master.move('R') moves the robot to the cell (0, 1).
// - master.isTarget() returns True.
// We now know that the target is the cell (0, 1), and the shortest path to the target is 2.

// **Example 2:**

// **Input:** grid = \[\[0,0,-1\],\[1,1,1\],\[2,0,0\]\]
// **Output:** 4
// **Explanation:** The minimum distance between the robot and the target is 4.

// **Example 3:**

// **Input:** grid = \[\[-1,0\],\[0,2\]\]
// **Output:** -1
// **Explanation:** There is no path from the robot to the target cell.

// **Constraints:**

// *   `m == grid.length`
// *   `n == grid[i].length`
// *   `1 <= n, m <= 500`
// *   `grid[i][j]` is either `-1`, `0`, `1`, or `2`.
// *   There is **exactly one** `-1` in `grid`.
// *   There is **exactly one** `2` in `grid`.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Unknown](https://leetcode.ca/tags/#Unknown)

// int find_shortest_path(GridMaster master) {
fn d2xy(direction: char) -> (i32, i32) {
    match direction {
        'U' => (-1, 0),
        'D' => (1, 0),
        'L' => (0, -1),
        'R' => (0, 1),
        _ => (0, 0),
    }
}
pub struct GridMaster {
    grid: Vec<Vec<i32>>,
    curr: (i32, i32),
    target: (i32, i32),
}
impl GridMaster {
    pub fn new(grid: Vec<Vec<i32>>) -> Self {
        let mut curr = (0, 0);
        let mut target = (0, 0);
        for (i, row) in grid.iter().enumerate() {
            for (j, &v) in row.iter().enumerate() {
                if v == -1 {
                    curr = (i as i32, j as i32);
                } else if v == 2 {
                    target = (i as i32, j as i32);
                }
            }
        }
        Self { grid, curr, target }
    }
    pub fn can_move(&self, direction: char) -> bool {
        let (dx, dy) = d2xy(direction);
        let (x, y) = self.curr;
        let (nx, ny) = (x + dx, y + dy);
        if nx < 0 || nx >= self.grid.len() as i32 || ny < 0 || ny >= self.grid[0].len() as i32 {
            return false;
        }
        self.grid[nx as usize][ny as usize] != 0
    }
    pub fn moving(&mut self, direction: char) {
        if self.can_move(direction) {
            let (dx, dy) = d2xy(direction);
            self.curr.0 += dx;
            self.curr.1 += dy;
        }
    }
    pub fn is_target(&self) -> bool {
        self.curr == self.target
    }
}
#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn find_shortest_path(master: GridMaster) -> i32 {
        let mut master = master;
        use std::collections::HashSet;
        let max = 500;
        let mut grid = vec![vec![-2; max * 2]; max * 2];
        fn dfs(x: i32, y: i32, grid: &mut Vec<Vec<i32>>, master: &mut GridMaster) {
            if grid[x as usize][y as usize] != -2 {
                return;
            }
            grid[x as usize][y as usize] = if master.is_target() { 2 } else { 1 };
            for d in ['U', 'D', 'L', 'R'] {
                let (dx, dy) = d2xy(d);
                let (nx, ny) = (x + dx, y + dy);
                if master.can_move(d) {
                    master.moving(d);
                    dfs(nx, ny, grid, master);
                    let rd = match d {
                        'U' => 'D',
                        'D' => 'U',
                        'L' => 'R',
                        'R' => 'L',
                        _ => ' ',
                    };
                    master.moving(rd);
                } else {
                    grid[nx as usize][ny as usize] = 0;
                }
            }
        }
        dfs(max as i32, max as i32, &mut grid, &mut master);
        let mut steps = 0;
        let mut q = std::collections::VecDeque::from([(max as i32, max as i32)]);
        grid[max][max] = 0;
        let dirs = [0, 1, 0, -1, 0];
        while !q.is_empty() {
            steps += 1;
            let n = q.len();
            for _ in 0..n {
                let (x, y) = q.pop_front().unwrap();
                for (dx, dy) in dirs[..dirs.len() - 1].iter().zip(&dirs[1..]) {
                    let (nx, ny) = (x + dx, y + dy);
                    if grid[nx as usize][ny as usize] == 2 {
                        return steps;
                    }
                    if grid[nx as usize][ny as usize] == 0 {
                        continue;
                    }
                    grid[nx as usize][ny as usize] = 0;
                    q.push_back((nx, ny));
                }
            }
        }
        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_find_shortest_path_1() {
        let master = GridMaster::new(vec![vec![1, 2], vec![-1, 0]]);
        assert_eq!(2, Solution::find_shortest_path(master,));
    }
    #[test]
    pub fn test_find_shortest_path_2() {
        let master = GridMaster::new(vec![vec![0, 0, -1], vec![1, 1, 1], vec![2, 0, 0]]);
        assert_eq!(4, Solution::find_shortest_path(master,));
    }
    #[test]
    pub fn test_find_shortest_path_3() {
        let master = GridMaster::new(vec![vec![-1, 0], vec![0, 2]]);
        assert_eq!(-1, Solution::find_shortest_path(master,));
    }
}

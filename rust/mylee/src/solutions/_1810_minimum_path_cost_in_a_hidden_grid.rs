// 1810\. Minimum Path Cost in a Hidden Grid
// =========================================

// This is an **interactive problem**.

// There is a robot in a hidden grid, and you are trying to get it from its starting cell to the target cell in this grid.
// The grid is of size `m x n`, and each cell in the grid is either empty or blocked.
// It is **guaranteed** that the starting cell and the target cell are different, and neither of them is blocked.

// Each cell has a **cost** that you need to pay each time you **move** to the cell.
// The starting cell's cost is **not** applied before the robot moves.

// You want to find the minimum total cost to move the robot to the target cell.
// However, you **do not know** the grid's dimensions, the starting cell, nor the target cell. You are only allowed to ask queries to the `GridMaster` object.

// The `GridMaster` class has the following functions:

// *   `boolean canMove(char direction)` Returns `true` if the robot can move in that direction. Otherwise, it returns `false`.
// *   `int move(char direction)` Moves the robot in that direction and returns the cost of moving to that cell.
// If this move would move the robot to a blocked cell or off the grid, the move will be **ignored**, the robot will remain in the same position, and the function will return `-1`.
// *   `boolean isTarget()` Returns `true` if the robot is currently on the target cell. Otherwise, it returns `false`.

// Note that `direction` in the above functions should be a character from `{'U','D','L','R'}`, representing the directions up, down, left, and right, respectively.

// Return _the **minimum total cost** to get the robot from its initial starting cell to the target cell. If there is no valid path between the cells, return_ `-1`.

// **Custom testing:**

// The test input is read as a 2D matrix `grid` of size `m x n` and four integers `r1`, `c1`, `r2`, and `c2` where:

// *   `grid[i][j] == 0` indicates that the cell `(i, j)` is blocked.
// *   `grid[i][j] >= 1` indicates that the cell `(i, j)` is empty and `grid[i][j]` is the **cost** to move to that cell.
// *   `(r1, c1)` is the starting cell of the robot.
// *   `(r2, c2)` is the target cell of the robot.

// Remember that you will **not** have this information in your code.

// **Example 1:**

// **Input:** grid = \[\[2,3\],\[1,1\]\], r1 = 0, c1 = 1, r2 = 1, c2 = 0
// **Output:** 2
// **Explanation:** One possible interaction is described below:
// The robot is initially standing on cell (0, 1), denoted by the 3.
// - master.canMove('U') returns false.
// - master.canMove('D') returns true.
// - master.canMove('L') returns true.
// - master.canMove('R') returns false.
// - master.move('L') moves the robot to the cell (0, 0) and returns 2.
// - master.isTarget() returns false.
// - master.canMove('U') returns false.
// - master.canMove('D') returns true.
// - master.canMove('L') returns false.
// - master.canMove('R') returns true.
// - master.move('D') moves the robot to the cell (1, 0) and returns 1.
// - master.isTarget() returns true.
// - master.move('L') doesn't move the robot and returns -1.
// - master.move('R') moves the robot to the cell (1, 1) and returns 1.
// We now know that the target is the cell (0, 1), and the minimum total cost to reach it is 2.

// **Example 2:**

// **Input:** grid = \[\[0,3,1\],\[3,4,2\],\[1,2,0\]\], r1 = 2, c1 = 0, r2 = 0, c2 = 2
// **Output:** 9
// **Explanation:** The minimum cost path is (2,0) -> (2,1) -> (1,1) -> (1,2) -> (0,2).

// **Example 3:**

// **Input:** grid = \[\[1,0\],\[0,1\]\], r1 = 0, c1 = 0, r2 = 1, c2 = 1
// **Output:** -1
// **Explanation:** There is no path from the robot to the target cell.

// **Constraints:**

// *   `1 <= n, m <= 100`
// *   `m == grid.length`
// *   `n == grid[i].length`
// *   `0 <= grid[i][j] <= 100`

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Facebook](https://leetcode.ca/tags/#Facebook) [Google](https://leetcode.ca/tags/#Google)
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
    pub fn new(grid: Vec<Vec<i32>>, curr: (i32, i32), target: (i32, i32)) -> Self {
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
    pub fn moving(&mut self, direction: char) -> i32 {
        if self.can_move(direction) {
            let (dx, dy) = d2xy(direction);
            self.curr.0 += dx;
            self.curr.1 += dy;
            return self.grid[self.curr.0 as usize][self.curr.1 as usize];
        }
        -1
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
        let max = 100;
        let mut grid = vec![vec![-1; max * 2]; max * 2];
        fn dfs(
            x: i32,
            y: i32,
            cost: i32,
            grid: &mut Vec<Vec<i32>>,
            master: &mut GridMaster,
            target: &mut Vec<i32>,
        ) {
            if grid[x as usize][y as usize] != -1 {
                return;
            }
            if master.is_target() {
                *target = vec![x, y];
            }
            grid[x as usize][y as usize] = cost;
            for d in ['U', 'D', 'L', 'R'] {
                let (dx, dy) = d2xy(d);
                let (nx, ny) = (x + dx, y + dy);
                if grid[nx as usize][ny as usize] != -1 {
                    continue;
                }

                if master.can_move(d) {
                    let cost = master.moving(d);
                    dfs(nx, ny, cost, grid, master, target);
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
        let mut target = Vec::new();
        dfs(
            max as i32,
            max as i32,
            1,
            &mut grid,
            &mut master,
            &mut target,
        );
        if target.is_empty() {
            return -1;
        }
        use std::cmp::Reverse;
        let mut q = std::collections::BinaryHeap::from([Reverse((0, max as i32, max as i32))]);
        grid[max][max] = 0;
        let dirs = [0, 1, 0, -1, 0];
        while let Some(Reverse((c, x, y))) = q.pop() {
            if x == target[0] && y == target[1] {
                return c;
            }
            grid[x as usize][y as usize] = 0;
            for (dx, dy) in dirs[..dirs.len() - 1].iter().zip(&dirs[1..]) {
                let (nx, ny) = (x + dx, y + dy);
                if grid[nx as usize][ny as usize] <= 0 {
                    continue;
                }
                q.push(Reverse((grid[nx as usize][ny as usize] + c, nx, ny)));
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
        let master = GridMaster::new(vec![vec![2, 3], vec![1, 1]], (0, 1), (1, 0));
        assert_eq!(2, Solution::find_shortest_path(master));
    }
    #[test]
    pub fn test_find_shortest_path_2() {
        let master = GridMaster::new(
            vec![vec![0, 3, 1], vec![3, 4, 2], vec![1, 2, 0]],
            (2, 0),
            (0, 2),
        );
        assert_eq!(9, Solution::find_shortest_path(master));
    }
    #[test]
    pub fn test_find_shortest_path_3() {
        let master = GridMaster::new(vec![vec![1, 0], vec![0, 1]], (0, 0), (1, 1));
        assert_eq!(-1, Solution::find_shortest_path(master));
    }
}

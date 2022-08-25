// 1730\. Shortest Path to Get Food
// ================================

// You are starving and you want to eat food as quickly as possible. You want to find the shortest path to arrive at a food cell.

// You are given an `m x n` character matrix, `grid`, of these different types of cells:

// *   `'*'` is your location. There is **exactly one** `'*'` cell.
// *   `'#'` is a food cell. There may be **multiple** food cells.
// *   `'O'` is free space, and you can travel through these cells.
// *   `'X'` is an obstacle, and you cannot travel through these cells.

// Return _the length of the shortest path for you to reach **any** food cell_. If there is no path for you to reach food, return `-1`.

// **Example 1:**

// ![](https://assets.leetcode.com/uploads/2020/09/21/img1.jpg)

// **Input:** grid = \[\["X","X","X","X","X","X"\],\["X","\*","O","O","O","X"\],\["X","O","O","#","O","X"\],\["X","X","X","X","X","X"\]\]
// **Output:** 3
// **Explanation:** It takes 3 steps to reach the food.

// **Example 2:**

// ![](https://assets.leetcode.com/uploads/2020/09/21/img2.jpg)

// **Input:** grid = \[\["X","X","X","X","X"\],\["X","\*","X","O","X"\],\["X","O","X","#","X"\],\["X","X","X","X","X"\]\]
// **Output:** -1
// **Explanation:** It is not possible to reach the food.

// **Example 3:**

// ![](https://assets.leetcode.com/uploads/2020/09/21/img3.jpg)

// **Input:** grid = \[\["X","X","X","X","X","X","X","X"\],\["X","\*","O","X","O","#","O","X"\],\["X","O","O","X","O","O","X","X"\],\["X","O","O","O","O","#","O","X"\],\["X","X","X","X","X","X","X","X"\]\]
// **Output:** 6
// **Explanation:** There can be multiple food cells. It only takes 6 steps to reach the bottom food.

// **Example 4:**

// **Input:** grid = \[\["O","\*"\],\["#","O"\]\]
// **Output:** 2

// **Example 5:**

// **Input:** grid = \[\["X","\*"\],\["#","X"\]\]
// **Output:** -1

// **Constraints:**

// *   `m == grid.length`
// *   `n == grid[i].length`
// *   `1 <= m, n <= 200`
// *   `grid[row][col]` is `'*'`, `'X'`, `'O'`, or `'#'`.
// *   The `grid` contains **exactly one** `'*'`.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Unknown](https://leetcode.ca/tags/#Unknown)

//  int get_food(char[][] grid)

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn get_food(grid: Vec<Vec<char>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut start = (0, 0);
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == '*' {
                    start = (i as i32, j as i32);
                    break;
                }
            }
        }
        let mut q = std::collections::VecDeque::from([start]);
        let mut steps = 0;
        let dirs = [0, 1, 0, -1, 0];
        let mut visited = std::collections::HashSet::new();
        while !q.is_empty() {
            let qn = q.len();
            for _ in 0..qn {
                let (x, y) = q.pop_front().unwrap();
                for (dx, dy) in dirs[..dirs.len() - 1].iter().zip(&dirs[1..]) {
                    let (nx, ny) = (x + dx, y + dy);
                    if nx < 0
                        || nx >= m as i32
                        || ny < 0
                        || ny >= n as i32
                        || grid[nx as usize][ny as usize] == 'X'
                        || visited.contains(&(nx, ny))
                    {
                        continue;
                    }
                    if grid[nx as usize][ny as usize] == '#' {
                        return steps + 1;
                    }
                    visited.insert((nx, ny));
                    q.push_back((nx, ny));
                }
            }
            steps += 1;
        }
        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_get_food_1() {
        assert_eq!(
            3,
            Solution::get_food(vec![
                vec!['X', 'X', 'X', 'X', 'X', 'X'],
                vec!['X', '*', 'O', 'O', 'O', 'X'],
                vec!['X', 'O', 'O', '#', 'O', 'X'],
                vec!['X', 'X', 'X', 'X', 'X', 'X']
            ])
        );
    }
    #[test]
    pub fn test_get_food_2() {
        assert_eq!(
            -1,
            Solution::get_food(vec![
                vec!['X', 'X', 'X', 'X', 'X'],
                vec!['X', '*', 'X', 'O', 'X'],
                vec!['X', 'O', 'X', '#', 'X'],
                vec!['X', 'X', 'X', 'X', 'X']
            ])
        );
    }
    #[test]
    pub fn test_get_food_3() {
        assert_eq!(
            6,
            Solution::get_food(vec![
                vec!['X', 'X', 'X', 'X', 'X', 'X', 'X', 'X'],
                vec!['X', '*', 'O', 'X', 'O', '#', 'O', 'X'],
                vec!['X', 'O', 'O', 'X', 'O', 'O', 'X', 'X'],
                vec!['X', 'O', 'O', 'O', 'O', '#', 'O', 'X'],
                vec!['X', 'X', 'X', 'X', 'X', 'X', 'X', 'X']
            ])
        );
    }
    #[test]
    pub fn test_get_food_4() {
        assert_eq!(2, Solution::get_food(vec![vec!['O', '*'], vec!['#', 'O']]));
    }
    #[test]
    pub fn test_get_food_5() {
        assert_eq!(-1, Solution::get_food(vec![vec!['X', '*'], vec!['#', 'X']]));
    }
}

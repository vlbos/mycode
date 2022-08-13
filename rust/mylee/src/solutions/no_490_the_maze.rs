// 490\. The Maze
// ==============

// There is a **ball** in a maze with empty spaces and walls. The ball can go through empty spaces by rolling **up**, **down**, **left** or **right**,
//  but it won't stop rolling until hitting a wall. When the ball stops, it could choose the next direction.

// Given the ball's **start position**, the **destination** and the **maze**, determine whether the ball could stop at the destination.

// The maze is represented by a binary 2D array. 1 means the wall and 0 means the empty space. You may assume that the borders of the maze are all walls.
// The start and destination coordinates are represented by row and column indexes.

// **Example 1:**

// **Input 1:** a maze represented by a 2D array

// 0 0 1 0 0
// 0 0 0 0 0
// 0 0 0 1 0
// 1 1 0 1 1
// 0 0 0 0 0

// **Input 2:** start coordinate (rowStart, colStart) = (0, 4)
// **Input 3:** destination coordinate (rowDest, colDest) = (4, 4)

// **Output:** true

// **Explanation:** One possible way is : left -> down -> left -> down -> right -> down -> right.
// ![](https://assets.leetcode.com/uploads/2018/10/12/maze_1_example_1.png)

// **Example 2:**

// **Input 1:** a maze represented by a 2D array

// 0 0 1 0 0
// 0 0 0 0 0
// 0 0 0 1 0
// 1 1 0 1 1
// 0 0 0 0 0

// **Input 2:** start coordinate (rowStart, colStart) = (0, 4)
// **Input 3:** destination coordinate (rowDest, colDest) = (3, 2)

// **Output:** false

// **Explanation:** There is no way for the ball to stop at the destination.
// ![](https://assets.leetcode.com/uploads/2018/10/13/maze_1_example_2.png)

// **Note:**

// 1.  There is only one ball and one destination in the maze.
// 2.  Both the ball and the destination exist on an empty space, and they will not be at the same position initially.
// 3.  The given maze does not contain border (like the red rectangle in the example pictures), but you could assume the border of the maze are all walls.
// 4.  The maze contains at least 2 empty spaces, and both the width and height of the maze won't exceed 100.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [Facebook](https://leetcode.ca/tags/#Facebook) [Google](https://leetcode.ca/tags/#Google) [Microsoft](https://leetcode.ca/tags/#Microsoft) [Twitter](https://leetcode.ca/tags/#Twitter) [Uber](https://leetcode.ca/tags/#Uber)
// @lc code=start
// use std::collections::HashSet;
// use std::collections::VecDeque;

// type Pos = (isize, isize);
// type Dir = usize;

// const MOVEMENTS: &[Pos; 4] = &[(0, 1), (0, -1), (1, 0), (-1, 0)];

impl Solution {
    pub fn has_path(maze: Vec<Vec<i32>>, start: Vec<i32>, destination: Vec<i32>) -> bool {
        // let rows = maze.len();
        // let cols = if rows == 0 { 0 } else { maze[0].len() };
        // if rows == 0 || cols == 0 {
        //     return false;
        // };
        // let mut queue = VecDeque::<(isize, isize)>::new();
        // let mut set = HashSet::new();
        // let start = (start[0] as isize, start[1] as isize);
        // let dest = (destination[0] as isize, destination[1] as isize);
        // let start_valid = start.0 >= 0
        //     && start.1 >= 0
        //     && start.0 < (rows as isize)
        //     && start.1 < (cols as isize)
        //     && maze[start.0 as usize][start.1 as usize] == 0;
        // if !start_valid {
        //     return false;
        // }
        // if start == dest {
        //     return true;
        // }
        // queue.push_back(start);
        // set.insert(start);
        // while let Some((y, x)) = queue.pop_front() {
        //     'outer: for &(dy, dx) in MOVEMENTS {
        //         println!("x: {:?}, y: {:?}, dx: {:?}, dy: {:?}", x, y, dx, dy);
        //         let mut cx = x;
        //         let mut cy = y;
        //         loop {
        //             let nx = cx + dx;
        //             let ny = cy + dy;
        //             let next_valid = nx >= 0
        //                 && ny >= 0
        //                 && nx < (cols as isize)
        //                 && ny < (rows as isize)
        //                 && maze[ny as usize][nx as usize] == 0;
        //             let curr_pos = (cy, cx);
        //             if !next_valid {
        //                 if !(cx == x && cy == y) && !set.contains(&curr_pos) {
        //                     if curr_pos == dest {
        //                         return true;
        //                     }
        //                     queue.push_back(curr_pos);
        //                     set.insert(curr_pos);
        //                 }
        //                 continue 'outer;
        //             }
        //             cx = nx;
        //             cy = ny;
        //         }
        //     }
        // }
        // false
        pub fn dfs(maze: &mut Vec<Vec<i32>>, start: &Vec<i32>, destination: &Vec<i32>) -> bool {
            if start == destination {
                return true;
            }
            if maze[start[0] as usize][start[1] as usize] != 0 {
                return false;
            }
            maze[start[0] as usize][start[1] as usize] = -1;
            let (m, n) = (maze.len() as i32, maze[0].len() as i32);
            let dirs = [0, 1, 0, -1, 0];
            for (i, &d) in dirs[1..].iter().enumerate() {
                let (mut x, mut y) = (start[0], start[1]);
                while x + dirs[i] >= 0
                    && x + dirs[i] < m
                    && y + d >= 0
                    && y + d < n
                    && maze[(x + dirs[i]) as usize][(y + d) as usize] != 1
                {
                    x += dirs[i];
                    y += d;
                }
                if dfs(maze, &vec![x, y], destination) {
                    return true;
                }
            }
            false
        }
        let mut maze = maze;
        dfs(&mut maze, &start, &destination)
    }
}
// @lc code=end

#[allow(dead_code)]
pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_has_path_1() {
        let maze = vec![
            vec![0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 1, 0],
            vec![1, 1, 0, 1, 1],
            vec![0, 0, 0, 0, 0],
        ];
        assert_eq!(Solution::has_path(maze, vec![0, 4], vec![4, 4]), true);
    }

    #[test]
    pub fn test_has_path_2() {
        let maze = vec![
            vec![0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 1, 0],
            vec![1, 1, 0, 1, 1],
            vec![0, 0, 0, 0, 0],
        ];
        assert_eq!(Solution::has_path(maze, vec![0, 4], vec![3, 2]), false);
    }
}

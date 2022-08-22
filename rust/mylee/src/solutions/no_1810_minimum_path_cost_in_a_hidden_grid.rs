// 1810\. Minimum Path Cost in a Hidden Grid
// =========================================

// This is an **interactive problem**.

// There is a robot in a hidden grid, and you are trying to get it from its starting cell to the target cell in this grid. The grid is of size `m x n`, and each cell in the grid is either empty or blocked. It is **guaranteed** that the starting cell and the target cell are different, and neither of them is blocked.

// Each cell has a **cost** that you need to pay each time you **move** to the cell. The starting cell's cost is **not** applied before the robot moves.

// You want to find the minimum total cost to move the robot to the target cell. However, you **do not know** the grid's dimensions, the starting cell, nor the target cell. You are only allowed to ask queries to the `GridMaster` object.

// The `GridMaster` class has the following functions:

// *   `boolean canMove(char direction)` Returns `true` if the robot can move in that direction. Otherwise, it returns `false`.
// *   `int move(char direction)` Moves the robot in that direction and returns the cost of moving to that cell. If this move would move the robot to a blocked cell or off the grid, the move will be **ignored**, the robot will remain in the same position, and the function will return `-1`.
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

use super::util::tree::TreeNode;

#[allow(dead_code)]
pub struct Solution {}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn check_equivalence(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        use std::collections::HashMap;
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, v: i32, freq: &mut HashMap<i32, i32>) {
            if root.is_none() {
                return;
            }
            let node = root.as_ref().unwrap().borrow();
            if (node.val as u8 as char).is_ascii_alphabetic() {
                *freq.entry(node.val).or_insert(0) += v;
            } else {
                dfs(&node.left, v, freq);
                dfs(&node.right, v, freq);
            }
        }
        let mut freq = HashMap::new();
        dfs(&root1, 1, &mut freq);
        dfs(&root2, -1, &mut freq);
        if freq.values().any(|v| *v > 0) {
            false
        } else {
            true
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    // use crate::tree;
    use super::super::util::tree::to_tree;
    fn to_exp_tree(s: &str) -> Option<Rc<RefCell<TreeNode>>> {
        to_tree(
            s.split(',')
                .map(|x| {
                    if x == "null" {
                        None
                    } else {
                        Some(x.as_bytes()[0] as i32)
                    }
                })
                .collect::<Vec<Option<i32>>>(),
        )
    }
    #[test]
    pub fn test_check_equivalence_1() {
        assert!(Solution::check_equivalence(
            to_exp_tree("x"),
            to_exp_tree("x")
        ));
    }
    #[test]
    pub fn test_check_equivalence_2() {
        assert!(Solution::check_equivalence(
            to_exp_tree("+,a,+,null,null,b,c"),
            to_exp_tree("+,+,b,c,a")
        ));
    }
    #[test]
    pub fn test_check_equivalence_3() {
        assert!(!Solution::check_equivalence(
            to_exp_tree("+,a,+,null,null,b,c"),
            to_exp_tree("+,+,b,d,a")
        ));
    }
}

// 1778\. Shortest Path in a Hidden Grid
// =====================================

// This is an **interactive problem**.

// You are given a robot in a hidden grid, and it wants to go to a target cell in this grid. The grid is of size `m x n`, and each cell in the grid can be empty or blocked. It is **guaranteed** that the start point and the robot's destination are different, and neither of them is blocked.

// You want to find the robot's minimum distance to the target cell. However, you do not know the grid's dimensions, or the starting point of the robot, or its target destination. You are only allowed to ask queries to your `GridMaster` object.

// You are given a class `GridMaster` which you can call the following functions from:

// *   `boolean GridMaster.canMove(char direction)` returns `true` if the robot can move in that direction. Otherwise, it returns `false`.
// *   `void GridMaster.move(char direction)` moves the robot in that direction. If this move would move the robot to a blocked cell or off the grid, it will be **ignored,** and the robot would remain in the same position.
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

// int findShortestPath(GridMaster master) {

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

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

//  int getFood(char[][] grid)

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

/*
 * @lc app=leetcode id=1145 lang=rust
 *
 * [1145] Binary Tree Coloring Game
 */

// @lc code=start
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn btree_game_winning_move(root: Option<Rc<RefCell<TreeNode>>>, n: i32, x: i32) -> bool {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, x: i32, xl: &mut i32, xr: &mut i32) -> i32 {
            if let Some(n) = root {
                let l = dfs(&n.borrow().left, x, xl, xr);
                let r = dfs(&n.borrow().right, x, xl, xr);
                if n.borrow().val == x {
                    *xl = l;
                    *xr = r;
                }
                return l + r + 1;
            }
            0
        }
        let (mut xl, mut xr) = (0, 0);
        let sum = dfs(&root, x, &mut xl, &mut xr);
        let other = sum - xl - xr - 1;
        other > xl + xr || xl > other + xr || xr > other + xl
    }
}
// @lc code=end

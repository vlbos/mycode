/*
 * @lc app=leetcode id=1026 lang=rust
 *
 * [1026] Maximum Difference Between Node and Ancestor
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
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, min: i32, max: i32) -> i32 {
            if let Some(node) = root {
                let val = node.borrow().val;
                let min = val.min(min);
                let max = val.max(max);
                let lm = dfs(&node.borrow().left, min, max);
                let rm = dfs(&node.borrow().right, min, max);
                return lm.max(rm);
            }
            max - min
        }
        dfs(&root, i32::MAX, i32::MIN)
    }
}
// @lc code=end

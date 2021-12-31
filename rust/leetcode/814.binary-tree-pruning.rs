/*
 * @lc app=leetcode id=814 lang=rust
 *
 * [814] Binary Tree Pruning
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
    pub fn prune_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(n) = root {
            let mut node = n.borrow_mut();

            node.left = Self::prune_tree(node.left.clone());
            node.right = Self::prune_tree(node.right.clone());
            if node.val == 0 && node.left.is_none() && node.right.is_none() {
                return None;
            }
            return Some(n.clone());
        }
        root
    }
}
// @lc code=end

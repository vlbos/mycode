/*
 * @lc app=leetcode id=111 lang=rust
 *
 * [111] Minimum Depth of Binary Tree
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
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(n) = root {
            if n.borrow().left.is_none() && n.borrow().right.is_none() {
                return 1;
            }
            let mut min = i32::MAX;
            if !n.borrow().left.is_none() {
                min = min.min(Self::min_depth(n.borrow().left.clone()));
            }
            if !n.borrow().right.is_none() {
                min = min.min(Self::min_depth(n.borrow().right.clone()));
            }
            return 1 + min;
        }
        0 
    }
}
// @lc code=end


/*
 * @lc app=leetcode id=1325 lang=rust
 *
 * [1325] Delete Leaves With a Given Value
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
    pub fn remove_leaf_nodes(root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(n)=root{
              let mut node = n.borrow_mut();
              node.left = Self::remove_leaf_nodes(node.left.clone(),target);
              node.right = Self::remove_leaf_nodes(node.right.clone(),target);
              if node.val==target && node.left.is_none() && node.right.is_none(){
              return None;
              }
              return Some(n.clone());
        }
        None
    }
}
// @lc code=end


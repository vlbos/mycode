/*
 * @lc app=leetcode.cn id=965 lang=rust
 *
 * [965] 单值二叉树
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
    pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(n) = root {
            if let Some(ref l) = &n.borrow().left {
                if n.borrow().val != l.borrow().val {
                    return false;
                }
            }
            if let Some(ref r) = &n.borrow().right {
                if n.borrow().val != r.borrow().val {
                    return false;
                }
            }
            let l = Self::is_unival_tree(n.borrow().left.clone());
            if !l {
                return false;
            }
            let r = Self::is_unival_tree(n.borrow().right.clone());
            if !r {
                return false;
            }
        }
        true
    }
}
// @lc code=end

/*
 * @lc app=leetcode.cn id=226 lang=rust
 *
 * [226] 翻转二叉树
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
use std::mem;
use std::rc::Rc;
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn inner_invert_tree(root: Option<&RefCell<TreeNode>>) {
            if let Some(node) = root {
                let node_ref = &mut *node.borrow_mut();

                mem::swap(&mut node_ref.left, &mut node_ref.right);

                inner_invert_tree(node_ref.left.as_deref());
                inner_invert_tree(node_ref.right.as_deref());
            }
        }
        inner_invert_tree(root.as_deref());
        root
    }
}
// @lc code=end

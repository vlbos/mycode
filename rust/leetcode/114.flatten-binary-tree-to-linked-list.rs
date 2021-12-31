/*
 * @lc app=leetcode id=114 lang=rust
 *
 * [114] Flatten Binary Tree to Linked List
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
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if let Some(node) = root {
            let mut bm = node.borrow_mut();
            Self::flatten(&mut bm.left);
            if let Some(left) = bm.left.take() {
                let val = left.borrow().val;
                bm.right = Some(Rc::new(RefCell::new(TreeNode {
                    val,
                    left: left.borrow_mut().right.take(),
                    right: bm.right.take(),
                })));
            }
            Self::flatten(&mut bm.right);
        }
    }
}
// @lc code=end

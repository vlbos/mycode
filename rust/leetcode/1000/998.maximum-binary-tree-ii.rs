/*
 * @lc app=leetcode id=998 lang=rust
 *
 * [998] Maximum Binary Tree II
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
    pub fn insert_into_max_tree(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            if node.borrow().val < val {
                return Some(Rc::new(RefCell::new(TreeNode {
                    val,
                    left: Some(node),
                    right: None,
                })));
            }
            if node.borrow().right.is_none() {
                node.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(val))));
            } else {
                let r = Self::insert_into_max_tree(node.borrow().right.clone(), val);
                node.borrow_mut().right = r;
            }
            return Some(node);
        }
        root
    }
}
// @lc code=end

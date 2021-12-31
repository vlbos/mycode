/*
 * @lc app=leetcode id=938 lang=rust
 *
 * [938] Range Sum of BST
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
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        if let Some(n) = root {
            let mut v = n.borrow().val;
            if v < low || v > high {
                v = 0;
            }
            v += Self::range_sum_bst(n.borrow().left.clone(), low, high);
            v += Self::range_sum_bst(n.borrow().right.clone(), low, high);
            return v;
        }

        0
    }
}
// @lc code=end


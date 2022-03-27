/*
 * @lc app=leetcode.cn id=938 lang=rust
 *
 * [938] 二叉搜索树的范围和
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

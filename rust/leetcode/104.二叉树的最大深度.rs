/*
 * @lc app=leetcode.cn id=104 lang=rust
 *
 * [104] 二叉树的最大深度
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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(ref n) = root {
            return 1 + match (&(n.borrow().left), &(n.borrow().right)) {
                (Some(ref l), Some(ref r)) => i32::max(
                    Solution::max_depth(Some(Rc::clone(l))),
                    Solution::max_depth(Some(Rc::clone(r))),
                ),
                (Some(ref l), None) => Solution::max_depth(Some(Rc::clone(l))),
                (None, Some(ref r)) => Solution::max_depth(Some(Rc::clone(r))),
                (None, None) => 0,
            };
        }
        0
    }
}
// @lc code=end

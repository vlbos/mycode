/*
 * @lc app=leetcode.cn id=100 lang=rust
 *
 * [100] 相同的树
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
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (Some(ref _p), Some(ref _q)) => {
                if (*_p.borrow()).val != (*_q.borrow()).val {
                    return false;
                }
                let left = match (&(&_p.borrow().left), &(&_q.borrow().left)) {
                    (Some(ref _pl), Some(ref _ql)) => {
                        Solution::is_same_tree(Some(Rc::clone(_pl)), Some(Rc::clone(_ql)))
                    }
                    (None, None) => true,
                    _ => false,
                };
                let right = match (&(&_p.borrow().right), &(&_q.borrow().right)) {
                    (Some(ref _pl), Some(ref _ql)) => {
                        Solution::is_same_tree(Some(Rc::clone(_pl)), Some(Rc::clone(_ql)))
                    }
                    (None, None) => true,
                    _ => false,
                };
                left && right
            }
            (None, None) => true,
            _ => false,
        }
    }
}
// @lc code=end

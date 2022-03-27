/*
 * @lc app=leetcode.cn id=404 lang=rust
 *
 * [404] 左叶子之和
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
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn inner_sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>, is_left: bool) -> i32 {
            if root.is_none() {
                return 0;
            }
            let mut r = 0;
            if let Some(_n) = root {
                if _n.borrow().left.is_none() && _n.borrow().right.is_none() {
                    if is_left {
                        return _n.borrow().val;
                    } else {
                        return 0;
                    }
                }
                if _n.borrow().left.is_some() {
                    r += inner_sum_of_left_leaves(_n.borrow().left.clone(), true);
                }
                if _n.borrow().right.is_some() {
                    r += inner_sum_of_left_leaves(_n.borrow().right.clone(), false);
                }
            }
            r
        }
        if root.is_none() {
            return 0;
        }
        let mut r = 0;
        if let Some(_n) = root {
            if _n.borrow().left.is_some() {
                r += inner_sum_of_left_leaves(_n.borrow().left.clone(), true);
            }
            if _n.borrow().right.is_some() {
                r += inner_sum_of_left_leaves(_n.borrow().right.clone(), false);
            }
        }
        r
    }
}
// @lc code=end

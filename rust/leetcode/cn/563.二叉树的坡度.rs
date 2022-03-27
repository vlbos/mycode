/*
 * @lc app=leetcode.cn id=563 lang=rust
 *
 * [563] 二叉树的坡度
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
    pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn sum_tree(root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
            if let Some(_n) = root {
                let n = _n.borrow();
                let mut ls = (0, 0);
                let mut rs = (0, 0);
                if let Some(_l) = &n.left {
                    ls = sum_tree(&n.left);
                }
                if let Some(r) = &n.right {
                    rs = sum_tree(&n.right);
                }
                return (n.val + ls.0 + rs.0, (ls.0 - rs.0).abs() + ls.1 + rs.1);
            }
            (0, 0)
        }
        sum_tree(&root).1
    }
}
// @lc code=end

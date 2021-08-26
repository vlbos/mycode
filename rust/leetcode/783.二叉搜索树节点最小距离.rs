/*
 * @lc app=leetcode.cn id=783 lang=rust
 *
 * [783] 二叉搜索树节点最小距离
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
    pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn inner_min_diff_in_bst(
            root: &Option<Rc<RefCell<TreeNode>>>,
            pre: &mut i32,
            result: &mut i32,
        ) {
            if root.is_none() {
                return;
            }
            if let Some(n) = root {
                inner_min_diff_in_bst(&n.borrow().left, pre, result);
                if *pre >= 0 {
                    *result = (*result).min(n.borrow().val - *pre);
                }
                *pre = n.borrow().val;
                inner_min_diff_in_bst(&n.borrow().right, pre, result);
            }
        }
        let mut pre = -1;
        let mut result = i32::MAX;
        inner_min_diff_in_bst(&root, &mut pre, &mut result);
        result
    }
}
// @lc code=end

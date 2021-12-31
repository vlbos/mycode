/*
 * @lc app=leetcode id=572 lang=rust
 *
 * [572] Subtree of Another Tree
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
    pub fn is_subtree(root: Option<Rc<RefCell<TreeNode>>>, sub_root: Option<Rc<RefCell<TreeNode>>>) -> bool {
         fn check(
            root: &Option<Rc<RefCell<TreeNode>>>,
            sub_root: &Option<Rc<RefCell<TreeNode>>>,
        ) -> bool {
            match (root, sub_root) {
                (Some(n), Some(sr)) => {
                    let _n = n.borrow();
                    let _sr = sr.borrow();
                    _n.val == _sr.val && check(&_n.left, &_sr.left) && check(&_n.right, &_sr.right)
                }
                (Some(_), None) | (None, Some(_)) => false,
                _ => true,
            }
        }
        fn dfs(
            root: &Option<Rc<RefCell<TreeNode>>>,
            sub_root: &Option<Rc<RefCell<TreeNode>>>,
        ) -> bool {
            match (root, sub_root) {
                (Some(n), Some(sr)) => {
                    let _n = n.borrow();
                    let _sr = sr.borrow();
                    check(&root, &sub_root) || dfs(&_n.left, &sub_root) || dfs(&_n.right, &sub_root)
                }
                (None, Some(_)) => false,
                (Some(_), None) | (None, None) => true,
            }
        }
        dfs(&root, &sub_root)
    }
}
// @lc code=end


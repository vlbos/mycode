/*
 * @lc app=leetcode id=98 lang=rust
 *
 * [98] Validate Binary Search Tree
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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn is_valid_bst_inner(root: &Option<Rc<RefCell<TreeNode>>>,  min: i64,max:i64) -> bool {
            if let Some(n) = root {
                let val =  n.borrow().val  as i64;
                return val>=min && val<max && is_valid_bst_inner(&n.borrow().left, min,val) && is_valid_bst_inner(&n.borrow().right, val+1,max);
            }
            true
        }
        is_valid_bst_inner(&root, i64::MIN,i64::MAX )
    }
}
// @lc code=end

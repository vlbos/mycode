/*
 * @lc app=leetcode id=110 lang=rust
 *
 * [110] Balanced Binary Tree
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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn depth(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
            if let Some(n) = root {
                let lenl = depth(&(n.borrow().left));
                let lenr = depth(&(n.borrow().right));
                if -1 == lenr || -1 == lenl || (lenl - lenr).abs() > 1 {
                    return -1;
                }
                return 1 + i32::max(lenl, lenr);
            }
            0
        }
        depth(&root) != -1
    }
}
// @lc code=end

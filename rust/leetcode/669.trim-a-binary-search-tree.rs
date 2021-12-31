/*
 * @lc app=leetcode id=669 lang=rust
 *
 * [669] Trim a Binary Search Tree
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
    pub fn trim_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(nn)=root{
            let mut n = nn.borrow_mut();
             if n.val>high{
                return Self::trim_bst(n.left.clone(),low,high);
            }
            if n.val<low{
                return Self::trim_bst(n.right.clone(),low,high);
            }
            n.left=Self::trim_bst(n.left.clone(),low,high);
            n.right=Self::trim_bst(n.right.clone(),low,high);
            return Some(nn.clone());
        }
        root
    }
}
// @lc code=end


/*
 * @lc app=leetcode id=1008 lang=rust
 *
 * [1008] Construct Binary Search Tree from Preorder Traversal
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
    pub fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() {
            return None;
        }
        let val = preorder[0];
        let mut root = TreeNode::new(val);
        if let Some(i) = preorder[1..].iter().position(|&x| x > val) {
            root.right = Self::bst_from_preorder(preorder[i+1..].to_vec());
            if i > 0 {
                root.left = Self::bst_from_preorder(preorder[1..i+1].to_vec());
            }
        } else if preorder.len() > 1 {
            root.left = Self::bst_from_preorder(preorder[1..].to_vec());
        }
        Some(Rc::new(RefCell::new(root)))
    }
}
// @lc code=end

/*
 * @lc app=leetcode.cn id=257 lang=rust
 *
 * [257] 二叉树的所有路径
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
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        fn inner_binary_tree_paths(node: &TreeNode, base: &mut String, result: &mut Vec<String>) {
            use std::fmt::Write;

            let saved_len = base.len();

            write!(base, "{}", node.val).unwrap();

            match (node.left.as_deref(), node.right.as_deref()) {
                (None, None) => result.push(base.clone()),
                (None, Some(child)) | (Some(child), None) => {
                    base.push_str("->");

                    inner_binary_tree_paths(&child.borrow(), base, result);
                }
                (Some(left), Some(right)) => {
                    base.push_str("->");

                    inner_binary_tree_paths(&left.borrow(), base, result);
                    inner_binary_tree_paths(&right.borrow(), base, result);
                }
            }

            base.truncate(saved_len);
        }
        let mut result = Vec::new();

        if let Some(node) = root.as_deref() {
            inner_binary_tree_paths(&node.borrow(), &mut String::new(), &mut result);
        }

        result
    }
}
// @lc code=end

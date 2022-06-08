/*
 * @lc app=leetcode id=106 lang=rust
 *
 * [106] Construct Binary Tree from Inorder and Postorder Traversal
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
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
         let mut root = None; 

        fn build_tree_inner(
            postorder: &[i32],
            inorder: &[i32],
            mut root: &mut Option<Rc<RefCell<TreeNode>>>,
        ) {
            if postorder.is_empty() {
                *root = None;
                return;
            }
            let mut j = 0;
            if postorder.len() > 1 {
                for (i, v) in inorder.iter().enumerate() {
                    if *v == postorder[postorder.len()-1] {
                        j = i;
                        break;
                    }
                }
            }
            let mut p = TreeNode::new(postorder[postorder.len()-1]);
            build_tree_inner(&postorder[0..j], &inorder[..j], &mut p.left);
            build_tree_inner(&postorder[j..postorder.len()-1], &inorder[j + 1..], &mut p.right);
            *root = Some(Rc::new(RefCell::new(p)));
        }
        build_tree_inner(&postorder, &inorder, &mut root);
        root
    }
}
// @lc code=end


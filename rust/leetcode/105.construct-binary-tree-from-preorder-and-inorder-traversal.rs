/*
 * @lc app=leetcode id=105 lang=rust
 *
 * [105] Construct Binary Tree from Preorder and Inorder Traversal
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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut root = None; 

        fn build_tree_inner(
            preorder: &[i32],
            inorder: &[i32],
            mut root: &mut Option<Rc<RefCell<TreeNode>>>,
        ) {
            if preorder.is_empty() {
                *root = None;
                return;
            }
            let mut j = 0;
            if preorder.len() > 1 {
                for (i, v) in inorder.iter().enumerate() {
                    if *v == preorder[0] {
                        j = i;
                        break;
                    }
                }
            }
            let mut p = TreeNode::new(preorder[0]);
            build_tree_inner(&preorder[1..j + 1], &inorder[..j], &mut p.left);
            build_tree_inner(&preorder[j + 1..], &inorder[j + 1..], &mut p.right);
            *root = Some(Rc::new(RefCell::new(p)));
        }
        build_tree_inner(&preorder, &inorder, &mut root);
        root
    }
}
// @lc code=end

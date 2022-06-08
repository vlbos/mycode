/*
 * @lc app=leetcode id=889 lang=rust
 *
 * [889] Construct Binary Tree from Preorder and Postorder Traversal
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
    pub fn construct_from_pre_post(
        preorder: Vec<i32>,
        postorder: Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(
            preorder: &Vec<i32>,
            postorder: &Vec<i32>,
            i: usize,
            j: usize,
            len: usize,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if len == 0 {
                return None;
            }
            let mut root = Rc::new(RefCell::new(TreeNode::new(preorder[i])));
            if len == 1 {
                return Some(root);
            }
            let mut prelen = 1;
            for k in 1..len {
                if postorder[j + k - 1] == preorder[i+1] {
                    prelen = k;
                    break;
                }
            }
            root.borrow_mut().left = dfs(preorder, postorder, i + 1, j, prelen);
            root.borrow_mut().right = dfs(
                preorder,
                postorder,
                i + prelen + 1,
                j + prelen,
                len - 1 - prelen,
            );
            Some(root)
        }
        dfs(&preorder, &postorder, 0, 0, preorder.len())
    }
}
// @lc code=end

/*
 * @lc app=leetcode id=968 lang=rust
 *
 * [968] Binary Tree Cameras
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
    pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32, i32) {
            if let Some(node) = root {
                let (la, lb, lc) = dfs(&node.borrow().left);
                let (ra, rb, rc) = dfs(&node.borrow().right);
                let a = lc + rc + 1;
                let b = a.min((la + rb).min(ra + lb));
                let c = a.min(lb + rb);
                return (a, b, c);
            }
            (i32::MAX / 2, 0, 0)
        }
        dfs(&root).1
    }
}
// @lc code=end

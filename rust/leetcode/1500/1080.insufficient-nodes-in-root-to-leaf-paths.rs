/*
 * @lc app=leetcode id=1080 lang=rust
 *
 * [1080] Insufficient Nodes in Root to Leaf Paths
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
    pub fn sufficient_subset(
        root: Option<Rc<RefCell<TreeNode>>>,
        limit: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, s: i32, limit: i32) -> bool {
            if let Some(n) = root {
                let mut node = n.borrow_mut();
                let sv = node.val + s;
                if node.left.is_none() && node.right.is_none() {
                    return sv < limit;
                }
                let lf = dfs(&node.left, sv, limit);
                if lf {
                    node.left = None;
                }
                let rf = dfs(&node.right, sv, limit);
                if rf {
                    node.right = None;
                }
                return lf && rf;
            }
            true
        }
        let mut root = root;
        if dfs(&root, 0, limit) {
            return None;
        }
        root
    }
}
// @lc code=end

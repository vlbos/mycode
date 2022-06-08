/*
 * @lc app=leetcode id=124 lang=rust
 *
 * [124] Binary Tree Maximum Path Sum
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
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, ans: &mut i32)->i32  {
            if let Some(n) = root {
                let node = n.borrow();
                let l = dfs(&node.left, ans).max(0);
                let r = dfs(&node.right, ans).max(0);
                *ans = (*ans).max(node.val + l + r);
                return node.val + l.max(r);
            }
            0
        }
        let mut ans = i32::MIN;
        dfs(&root, &mut ans);
        ans
    }
}
// @lc code=end

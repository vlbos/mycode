/*
 * @lc app=leetcode id=1372 lang=rust
 *
 * [1372] Longest ZigZag Path in a Binary Tree
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
    pub fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, dir: bool, len: i32, ans: &mut i32) {
            *ans = len.max(*ans);
            if let Some(n) = root {
                let node = n.borrow();
                if dir {
                    if node.right.is_some() {
                        dfs(&node.right, false, len + 1, ans);
                    }
                    if node.left.is_some() {
                        dfs(&node.left, true, 1, ans);
                    }
                } else {
                    if node.left.is_some() {
                        dfs(&node.left, true, len + 1, ans);
                    }
                    if node.right.is_some() {
                        dfs(&node.right, false, 1, ans);
                    }
                }
            }
        }
        if root.is_none() {
            return 0;
        }
        let mut ans = 0;
        dfs(&root, false, 0, &mut ans);
        dfs(&root, true, 0, &mut ans);
        ans
    }
}
// @lc code=end

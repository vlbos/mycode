/*
 * @lc app=leetcode id=979 lang=rust
 *
 * [979] Distribute Coins in Binary Tree
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
    pub fn distribute_coins(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, ans: &mut i32) -> i32 {
            if let Some(node) = root {
                let l = dfs(&node.borrow().left, ans);
                let r = dfs(&node.borrow().right, ans);
                *ans += l.abs() + r.abs();
                return node.borrow().val + l + r - 1;
            }
            0
        }
        dfs(&root, &mut ans);
        ans
    }
}
// @lc code=end

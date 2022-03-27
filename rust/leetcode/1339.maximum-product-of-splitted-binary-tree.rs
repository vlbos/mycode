/*
 * @lc app=leetcode id=1339 lang=rust
 *
 * [1339] Maximum Product of Splitted Binary Tree
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
    pub fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs_sum(root: &Option<Rc<RefCell<TreeNode>>>) -> i64 {
            if let Some(n) = root {
                let val = n.borrow().val as i64;
                let l = dfs_sum(&n.borrow().left);
                let r = dfs_sum(&n.borrow().right);
                return val + l + r;
            }
            0
        }
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, sum: i64, ans: &mut i64) -> i64 {
            if let Some(n) = root {
                let val = n.borrow().val as i64;

                let l = dfs(&n.borrow().left, sum, ans);
                let r = dfs(&n.borrow().right, sum, ans);
                let cursum = val + l + r;
                if (cursum * 2 - sum).abs() < ((*ans) * 2 - sum).abs() {
                    *ans = cursum;
                }
                return cursum;
            }
            0
        }
        let mut ans = 0;
        let sum = dfs_sum(&root);
        dfs(&root, sum, &mut ans);
        ((ans * (sum - ans)) % 1000_000_007) as _
    }
}
// @lc code=end

/*
 * @lc app=leetcode id=1305 lang=rust
 *
 * [1305] All Elements in Two Binary Search Trees
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
    pub fn get_all_elements(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<i32> {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>) {
            if let Some(r) = root {
                dfs(&r.borrow().left, ans);
                ans.push(r.borrow().val);
                dfs(&r.borrow().right, ans);
            }
        }
        let (mut r1, mut r2) = (Vec::new(), Vec::new());

        dfs(&root1, &mut r1);
        dfs(&root2, &mut r2);
        let mut ans = Vec::new();
        let (mut i, mut j) = (0, 0);
        while i < r1.len() || j < r2.len() {
            if i < r1.len() && (j == r2.len() || r1[i] < r2[j]) {
                ans.push(r1[i]);
                i += 1;
            } else {
                ans.push(r2[j]);
                j += 1;
            }
        }
        ans
    }
}
// @lc code=end

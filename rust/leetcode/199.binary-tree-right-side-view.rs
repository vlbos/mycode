/*
 * @lc app=leetcode id=199 lang=rust
 *
 * [199] Binary Tree Right Side View
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
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn dps(root: &Option<Rc<RefCell<TreeNode>>>, depth: usize, mut ans: &mut Vec<i32>) {
            if let Some(n) = root {
                if depth < ans.len() {
                    ans[depth] = n.borrow().val;
                } else {
                    ans.push(n.borrow().val);
                }
                dps(&n.borrow().left, depth + 1, ans);
                dps(&n.borrow().right, depth + 1, ans);
            }
        }
        let mut ans = Vec::new();
        dps(&root, 0, &mut ans);
        ans
    }
}
// @lc code=end

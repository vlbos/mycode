/*
 * @lc app=leetcode id=102 lang=rust
 *
 * [102] Binary Tree Level Order Traversal
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
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return Vec::new();
        }
        let mut ans = Vec::new();
        let mut q = std::collections::VecDeque::new();
        q.push_back((root.clone(), 0));
        while let Some(p) = q.pop_front() {
            if let Some(n) = p.0 {
                q.push_back((n.borrow().left.clone(), p.1 + 1));
                q.push_back((n.borrow().right.clone(), p.1 + 1));
                if ans.len() <= p.1 {
                    ans.push(vec![n.borrow().val]);
                } else {
                    ans[p.1].push(n.borrow().val);
                }
            }
        }
        ans
    }
}
// @lc code=end

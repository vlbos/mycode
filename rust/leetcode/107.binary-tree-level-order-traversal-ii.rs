/*
 * @lc app=leetcode id=107 lang=rust
 *
 * [107] Binary Tree Level Order Traversal II
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
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return Vec::new();
        }
        let mut ans = Vec::new();
        let mut q = std::collections::VecDeque::new();
        q.push_back((root.clone(), 0));
        while let Some(p) = q.pop_front() {
            if let Some(n) = p.0 {
                q.push_back((n.borrow_mut().left.take(), p.1 + 1));
                q.push_back((n.borrow_mut().right.take(), p.1 + 1));
                if ans.len() <= p.1 {
                    ans.insert(0, vec![n.borrow().val]);
                } else {
                    let mut l = ans.len() - 1;
                    ans[l - p.1].push(n.borrow().val);
                }
            }
        }
        ans
    }
}
// @lc code=end

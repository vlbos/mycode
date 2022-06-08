/*
 * @lc app=leetcode id=145 lang=rust
 *
 * [145] Binary Tree Postorder Traversal
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
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
                let mut v = Vec::<i32>::new();
        let mut s = Vec::<Option<Rc<RefCell<TreeNode>>>>::new();
        s.push(root);
        while let Some(Some(n)) = s.pop() {
            if let Some(l) = &n.borrow().left {
                s.push(Some(Rc::clone(&l)));
            }
            if let Some(r) = &n.borrow().right {
                s.push(Some(Rc::clone(&r)));
            }
            v.insert(0, n.borrow().val);
        }
        v
    }
}
// @lc code=end


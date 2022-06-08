/*
 * @lc app=leetcode id=1028 lang=rust
 *
 * [1028] Recover a Tree From Preorder Traversal
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
    pub fn recover_from_preorder(traversal: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut path: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();
        let mut pos = 0;
        let tb = traversal.as_bytes();
        let n = tb.len();
        while pos < n {
            let mut level = 0;
            while tb[pos] == b'-' {
                level += 1;
                pos += 1;
            }
            let mut value = 0;
            while pos < n && (tb[pos] as char).is_ascii_digit() {
                value = value * 10 + (tb[pos] - b'0') as i32;
                pos += 1;
            }
            let node = Some(Rc::new(RefCell::new(TreeNode::new(value))));
            if level == path.len() {
                if !path.is_empty() {
                    path.last_mut().unwrap().as_mut().unwrap().borrow_mut().left = node.clone();
                }
            } else {
                while level != path.len() {
                    path.pop();
                }
                path.last_mut()
                    .unwrap()
                    .as_mut()
                    .unwrap()
                    .borrow_mut()
                    .right = node.clone();
            }
            path.push(node);
        }
        path.remove(0)
    }
}
// @lc code=end

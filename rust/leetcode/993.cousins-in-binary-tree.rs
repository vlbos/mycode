/*
 * @lc app=leetcode id=993 lang=rust
 *
 * [993] Cousins in Binary Tree
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
    pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
        if let Some(ref n) = &root {
            if n.borrow().val == x || n.borrow().val == y {
                return false;
            }
        }
        let mut q = Vec::new();
        let mut d = Vec::new();
        let mut r = Vec::new();
        let mut m = std::collections::HashMap::new();
        let mut destdepth = -1;
        q.push(root.clone());
        d.push(0);
        while let Some(node) = q.pop() {
            let depth = d.pop().unwrap();
            if let Some(ref n) = &node {
                if (destdepth == -1 && (n.borrow().val == x || n.borrow().val == y))
                    || destdepth == depth
                {
                    r.push(n.borrow().val);
                    destdepth = depth;
                }
                if let Some(l) = &n.borrow().left {
                    q.insert(0, n.borrow().left.clone());
                    d.insert(0, depth + 1);
                    m.insert(l.borrow().val, n.borrow().val);
                }
                if let Some(r) = &n.borrow().right {
                    q.insert(0, n.borrow().right.clone());
                    d.insert(0, depth + 1);
                    m.insert(r.borrow().val, n.borrow().val);
                }
            }
        }
        r.contains(&x) && r.contains(&y) && m.get(&x).unwrap() != m.get(&y).unwrap()
    }
}
// @lc code=end

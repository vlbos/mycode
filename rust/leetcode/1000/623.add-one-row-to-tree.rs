/*
 * @lc app=leetcode id=623 lang=rust
 *
 * [623] Add One Row to Tree
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
    pub fn add_one_row(root: Option<Rc<RefCell<TreeNode>>>, val: i32, depth: i32) -> Option<Rc<RefCell<TreeNode>>> {
fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, val: i32, depth: i32) {
        if let Some(n) = node {
            if depth > 0 {
                dfs(&n.borrow().left, val, depth - 1);
                dfs(&n.borrow().right, val, depth - 1);
            } else {
                let mut n = n.borrow_mut();
                n.left = Some(Rc::new(RefCell::new(TreeNode {
                    val,
                    left: n.left.take(),
                    right: None,
                })));
                n.right = Some(Rc::new(RefCell::new(TreeNode {
                    val,
                    left: None,
                    right: n.right.take(),
                })));
            }
        }
    }
         if depth == 1 {
            Some(Rc::new(RefCell::new(TreeNode {
                val,
                left: root,
                right: None,
            })))
        } else {
            dfs(&root, val, depth - 2);
            root
        }

    }
}
// @lc code=end


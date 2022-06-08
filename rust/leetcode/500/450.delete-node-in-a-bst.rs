/*
 * @lc app=leetcode id=450 lang=rust
 *
 * [450] Delete Node in a BST
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
    pub fn delete_node(
        root: Option<Rc<RefCell<TreeNode>>>,
        key: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn helper(node: &Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
            fn search_next(node: &Option<Rc<RefCell<TreeNode>>>) -> Option<i32> {
                if let Some(n) = node {
                    if n.borrow().left.is_some() {
                        search_next(&n.borrow().left)
                    } else {
                        Some(n.borrow().val)
                    }
                } else {
                    None
                }
            }
            if let Some(n) = node {
                let val = n.borrow().val;
                match val.cmp(&key) {
                    std::cmp::Ordering::Greater => {
                        let l = helper(&n.borrow().left, key);
                        n.borrow_mut().left = l;
                    }
                    std::cmp::Ordering::Less => {
                        let r = helper(&n.borrow().right, key);
                        n.borrow_mut().right = r;
                    }
                    std::cmp::Ordering::Equal => {
                        if n.borrow().left.is_none() {
                            return n.borrow().right.clone();
                        }
                        if n.borrow().right.is_none() {
                            return n.borrow().left.clone();
                        }
                        let next = search_next(&n.borrow().right);
                        if let Some(val) = next {
                            let r = helper(&n.borrow().right, val);
                            n.borrow_mut().val = val;
                            n.borrow_mut().right = r;
                        }
                    }
                }
            }
            node.clone()
        }
        helper(&root, key)
    }
}
// @lc code=end

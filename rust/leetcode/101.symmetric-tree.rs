/*
 * @lc app=leetcode id=101 lang=rust
 *
 * [101] Symmetric Tree
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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }
        let mut q = std::collections::VecDeque::<Option<Rc<RefCell<TreeNode>>>>::new();
        let n = root;
        if let Some(ref _n) = n {
            let _ = match (&(_n.borrow().left), &(_n.borrow().right)) {
                (Some(ref l), Some(ref r)) => {
                    if &l.borrow().val == &r.borrow().val {
                        match (&(l.borrow().left), &(r.borrow().right)) {
                            (Some(lleft), Some(rright)) => {
                                q.push_back(Some(Rc::clone(lleft)));
                                q.push_back(Some(Rc::clone(rright)));
                                false
                            }
                            (None, None) => true,
                            _ => return false,
                        };
                        match (&(r.borrow().left), &(l.borrow().right)) {
                            (Some(rleft), Some(lright)) => {
                                q.push_back(Some(Rc::clone(lright)));
                                q.push_back(Some(Rc::clone(rleft)));
                                false
                            }
                            (None, None) => true,
                            _ => return false,
                        };
                    } else {
                        return false;
                    }
                    false
                }
                (None, None) => true,
                _ => return false,
            };
            while !q.is_empty() {
                let first = q.pop_front();
                let second = q.pop_front();

                let _ = match (first, second) {
                    (Some(ll), Some(rr)) => {
                        match (ll, rr) {
                            (Some(l), Some(r)) => {
                                if (l).borrow().val == (*r).borrow().val {
                                    match (&(l.borrow().left), &(r.borrow().right)) {
                                        (Some(lleft), Some(rright)) => {
                                            q.push_back(Some(Rc::clone(lleft)));
                                            q.push_back(Some(Rc::clone(rright)));
                                            false
                                        }
                                        (None, None) => true,
                                        _ => return false,
                                    };
                                    match (&(r.borrow().left), &(l.borrow().right)) {
                                        (Some(rleft), Some(lright)) => {
                                            q.push_back(Some(Rc::clone(lright)));
                                            q.push_back(Some(Rc::clone(rleft)));
                                            false
                                        }
                                        (None, None) => true,
                                        _ => return false,
                                    };
                                } else {
                                    // false
                                    return false;
                                }
                                false
                            }
                            _ => return false,
                        }
                    }
                    (None, None) => true,
                    _ => {
                        return false;
                    }
                };
            }
        }
        q.is_empty()
    }
}
// @lc code=end


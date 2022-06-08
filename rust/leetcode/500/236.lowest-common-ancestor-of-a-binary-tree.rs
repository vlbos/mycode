/*
 * @lc app=leetcode id=236 lang=rust
 *
 * [236] Lowest Common Ancestor of a Binary Tree
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
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut stack = Vec::new();
        fn pre_order(
            root: &Option<Rc<RefCell<TreeNode>>>,
            p: &Option<Rc<RefCell<TreeNode>>>,
            q: &Option<Rc<RefCell<TreeNode>>>,
            s: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
        ) -> i32 {
            if let Some(n) = root {
                s.push(root.clone());
                if n.borrow().val == p.as_ref().unwrap().borrow().val
                    || n.borrow().val == q.as_ref().unwrap().borrow().val
                {
                    if n.borrow().val != p.as_ref().unwrap().borrow().val {
                        return 2;
                    } else {
                        return 1;
                    }
                }
                let l = pre_order(&n.borrow().left, p, q, s);
                if l > 0 {
                    return l;
                }
                let r = pre_order(&n.borrow().right, p, q, s);
                if r > 0 {
                    return r;
                }
            }
            0
        }
        fn pre_order_s(
            root: &Option<Rc<RefCell<TreeNode>>>,
            p: &Option<Rc<RefCell<TreeNode>>>,
        ) -> bool {
            if let Some(n) = root {
                if n.borrow().val == p.as_ref().unwrap().borrow().val {
                    return true;
                }
                let l = pre_order_s(&n.borrow().left, p);
                if l {
                    return l;
                }
                let r = pre_order_s(&n.borrow().right, p);
                if r {
                    return r;
                }
            }
            false
        }
        let r = pre_order(&root, &p, &q, &mut stack);
        let r = if r == 1 { q } else { p };
        while let Some(n) = stack.pop() {
            let b = pre_order_s(&n, &r);
            if b {
                return n;
            }
        }
        None
    }
}
// @lc code=end

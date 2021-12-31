/*
 * @lc app=leetcode id=687 lang=rust
 *
 * [687] Longest Univalue Path
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
    pub fn longest_univalue_path(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn arrow_length(root: &Option<Rc<RefCell<TreeNode>>>, ans: &mut i32) -> i32 {
            if let Some(node) = root {
                let n = node.borrow();
                let ll = arrow_length(&n.left, ans);
                let rl = arrow_length(&n.right, ans);
                let mut al = 0;
                let mut ar = 0;
                if let Some(l) = &n.left {
                    if l.borrow().val == n.val {
                        al = ll + 1;
                    }
                }
                if let Some(r) = &n.right {
                    if r.borrow().val == n.val {
                        ar = rl + 1;
                    }
                }
                *ans = (*ans).max(al + ar);
                return al.max(ar);
            }
            0
        }
        let mut ans = 0;
        arrow_length(&root, &mut ans);
        ans
    }
}
// @lc code=end

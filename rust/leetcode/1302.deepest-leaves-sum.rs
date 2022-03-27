/*
 * @lc app=leetcode id=1302 lang=rust
 *
 * [1302] Deepest Leaves Sum
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
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(
            root: &Option<Rc<RefCell<TreeNode>>>,
            depth: i32,
            max_depth: &mut i32,
            ans: &mut i32,
        ) {
            if let Some(n) = root {
                if n.borrow().left.is_none() && n.borrow().right.is_none() {
                    if depth > *max_depth {
                        *max_depth = depth;
                        *ans = n.borrow().val;
                    } else if depth == *max_depth {
                        *ans += n.borrow().val;
                    }
                    return;
                }
                if n.borrow().left.is_some() {
                    dfs(&n.borrow().left, depth + 1, max_depth, ans);
                }
                if n.borrow().right.is_some() {
                    dfs(&n.borrow().right, depth + 1, max_depth, ans);
                }
            }
        }
        let mut ans = 0;
        dfs(&root, 0, &mut -1, &mut ans);
        ans
    }
}
// @lc code=end

/*
 * @lc app=leetcode.cn id=637 lang=rust
 *
 * [637] 二叉树的层平均值
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
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut q = Vec::<Rc<RefCell<TreeNode>>>::new();
        let mut r = Vec::<f64>::new();
        if let Some(_n) = root {
            q.push(Rc::clone(&_n));
        }
        while !q.is_empty() {
            let mut sum = 0f64;
            let size = q.len();
            let qq = q.clone();
            for n in qq {
                let _n = n.borrow();
                sum += _n.val as f64;
                if let Some(_l) = &_n.left {
                    q.push(Rc::clone(&_l));
                }
                if let Some(_r) = &_n.right {
                    q.push(Rc::clone(&_r));
                }
                q.remove(0);
            }
            r.push(sum / size as f64);
        }
        r
    }
}
// @lc code=end

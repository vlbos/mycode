/*
 * @lc app=leetcode id=437 lang=rust
 *
 * [437] Path Sum III
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
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        fn path_sum_count(root: &Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
            let mut sum = 0;
            if let Some(n) = root {
                if n.borrow().val == target_sum {
                    sum += 1;
                }
                sum += path_sum_count(&n.borrow().left, target_sum - n.borrow().val);
                sum += path_sum_count(&n.borrow().right, target_sum - n.borrow().val);
            }
            sum
        }

        let mut sum = path_sum_count(&root, target_sum);
        if let Some(n) = root {
            sum += Self::path_sum(n.borrow().left.clone(), target_sum);
            sum += Self::path_sum(n.borrow().right.clone(), target_sum);
        }
        sum
    }
}
// @lc code=end

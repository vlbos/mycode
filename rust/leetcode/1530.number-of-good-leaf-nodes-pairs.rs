/*
 * @lc app=leetcode id=1530 lang=rust
 *
 * [1530] Number of Good Leaf Nodes Pairs
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
    pub fn count_pairs(root: Option<Rc<RefCell<TreeNode>>>, distance: i32) -> i32 {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, distance: usize) -> (Vec<i32>, i32) {
            if let Some(n) = root {
                let node = n.borrow();
                let mut depths = vec![0; distance + 1];
                if node.left.is_none() && node.right.is_none() {
                    depths[0] = 1;
                    return (depths, 0);
                }

                let (ldepths, lc) = dfs(&node.left, distance);
                let (rdepths, rc) = dfs(&node.right, distance);
                for i in 0..distance {
                    depths[i + 1] += ldepths[i];
                    depths[i + 1] += rdepths[i];
                }
                let mut cnt = 0;
                for i in 0..=distance {
                    if i >= distance - 1 {
                        continue;
                    }
                    for j in 0..distance - i - 1 {
                        cnt += ldepths[i] * rdepths[j];
                    }
                }
                return (depths, cnt + lc + rc);
            }
            (vec![0; distance + 1], 0)
        }

        let (_, ans) = dfs(&root, distance as usize);
        ans
    }
}
// @lc code=end

/*
 * @lc app=leetcode id=655 lang=rust
 *
 * [655] Print Binary Tree
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
    pub fn print_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<String>> {
        fn height(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
            if let Some(n) = root {
                return 1 + height(&n.borrow().left).max(height(&n.borrow().right));
            }
            0
        }
        let h = height(&root);
        let m = h as usize;
        let n = 2usize.pow(h as u32) - 1;

        let mut ans = vec![vec![String::new(); n]; m];
        fn dfs(
            root: &Option<Rc<RefCell<TreeNode>>>,
            row: usize,
            col: usize,
            ans: &mut Vec<Vec<String>>,
        ) {
            if let Some(n) = root {
                let h = ans.len() - 1;
                ans[row][col] = n.borrow().val.to_string();
                let offset = 2usize.pow((h - row - 1) as u32);
                dfs(&n.borrow().left, row + 1, col - offset, ans);
                dfs(&n.borrow().right, row + 1, col + offset, ans);
            }
        }
        dfs(&root, 0, (n - 1) / 2, &mut ans);
        ans
    }
}
// @lc code=end

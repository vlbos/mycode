/*
 * @lc app=leetcode id=987 lang=rust
 *
 * [987] Vertical Order Traversal of a Binary Tree
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
    pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        use std::collections::BTreeMap;
        let mut ans = BTreeMap::new();
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, row:i32,col: i32, ans: &mut BTreeMap<i32, BTreeMap<i32,Vec<i32>>>) {
            if let Some(node) = root {
                ans.entry(col).or_insert(BTreeMap::new()).entry(row).or_insert(Vec::new()).push(node.borrow().val);
                dfs(&node.borrow().right, row+1, col + 1, ans);
                dfs(&node.borrow().left, row+1,col - 1, ans);
            }
        }
        dfs(&root, 0,0, &mut ans);
        ans.into_iter().map(|c|c.1.into_iter().map(|r|{let mut s=r.1.into_iter().collect::<Vec<i32>>();s.sort();s}).flatten().collect()).collect()
    }
}
// @lc code=end

/*
 * @lc app=leetcode id=1110 lang=rust
 *
 * [1110] Delete Nodes And Return Forest
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
    pub fn del_nodes(
        root: Option<Rc<RefCell<TreeNode>>>,
        to_delete: Vec<i32>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        use std::collections::HashSet;
        let mut to_deletes = to_delete.iter().cloned().collect::<HashSet<_>>();
        fn dfs(
            root: &Option<Rc<RefCell<TreeNode>>>,
            to_deletes: &mut HashSet<i32>,
            ans: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(n) = root {
                let mut node = n.borrow_mut();
                let left = dfs(&node.left, to_deletes, ans);
                let right = dfs(&node.right, to_deletes, ans);
                if !to_deletes.contains(&node.val) {
                    node.left = left;
                    node.right = right;
                    return Some(n.clone());
                }
                if left.is_some() {
                    ans.push(left);
                }
                if right.is_some() {
                    ans.push(right);
                }
                to_deletes.remove(&node.val);
            }
            None
        }
        let mut ans = Vec::new();
        let r = dfs(&root, &mut to_deletes, &mut ans);
        if r.is_some() {
            ans.push(r);
        }

        ans
    }
}
// @lc code=end

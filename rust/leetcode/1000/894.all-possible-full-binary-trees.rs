/*
 * @lc app=leetcode id=894 lang=rust
 *
 * [894] All Possible Full Binary Trees
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
    pub fn all_possible_fbt(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut m = std::collections::HashMap::new();
        if !m.contains_key(&n) {
            let mut ans = Vec::new();
            if n == 1 {
                ans.push(Some(Rc::new(RefCell::new(TreeNode::new(0)))));
            } else if n % 2 == 1 {
                for i in 0..n {
                    let j = n - 1 - i;
                    for l in &Self::all_possible_fbt(i) {
                        for r in &Self::all_possible_fbt(j) {
                            let mut node = TreeNode::new(0);
                            node.left = l.clone();
                            node.right = r.clone();
                            ans.push(Some(Rc::new(RefCell::new(node))));
                        }
                    }
                }
            }
            m.insert(n, ans);
        }
        m.get(&n).unwrap().clone()
    }
}
// @lc code=end

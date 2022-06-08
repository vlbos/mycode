/*
 * @lc app=leetcode id=297 lang=rust
 *
 * [297] Serialize and Deserialize Binary Tree
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
struct Codec {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Self {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
         let mut ans = String::new();
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, ans: &mut String) {
            if let Some(r) = root {
                let node = r.borrow();
                ans.push_str((node.val.to_string()+",").as_str());
                dfs(&node.left, ans);
                dfs(&node.right, ans);
                return;
            }
            ans.push_str("None,");
        }
        dfs(&root, &mut ans);
        ans
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let d = data.split(',').collect::<Vec<&str>>();
        fn dfs(d: &Vec<&str>, i: &mut usize) -> Option<Rc<RefCell<TreeNode>>> {
            if *i >= d.len() || d[*i] == "None" {
                *i += 1;
                return None;
            }
            let mut node = TreeNode::new(d[*i].parse::<i32>().unwrap());
            *i += 1;
            node.left = dfs(d, i);
            node.right = dfs(d, i);
            Some(Rc::new(RefCell::new(node)))
        }
        let mut i = 0;
        dfs(&d, &mut i)
    }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data: String = obj.serialize(strs);
 * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
 */
// @lc code=end

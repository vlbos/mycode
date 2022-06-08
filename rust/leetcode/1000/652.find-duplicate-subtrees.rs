/*
 * @lc app=leetcode id=652 lang=rust
 *
 * [652] Find Duplicate Subtrees
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
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn find_duplicate_subtrees(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>,m:&mut std::collections::HashMap<String,i32>,ans:&mut Vec<Option<Rc<RefCell<TreeNode>>>>)->String{
            if let Some(n)=root{
                let serial = format!("{},{},{}",n.borrow().val,dfs(&n.borrow().left,m,ans),dfs(&n.borrow().right,m,ans));
                *m.entry(serial.clone()).or_insert(0)+=1;
                if let Some(cnt)=m.get(&serial){
                    if *cnt==2{
                        ans.push(root.clone());
                    }
                }
                return serial.clone();
            }
            "#".to_string()
        }
        let mut m = std::collections::HashMap::new();
        let mut ans =Vec::new();
        dfs(&root,&mut m,&mut ans);
        ans
    }
}
// @lc code=end


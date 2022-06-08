/*
 * @lc app=leetcode id=2096 lang=rust
 *
 * [2096] Step-By-Step Directions From a Binary Tree Node to Another
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
    pub fn get_directions(
        root: Option<Rc<RefCell<TreeNode>>>,
        start_value: i32,
        dest_value: i32,
    ) -> String {
        use std::collections::HashMap;
        let mut m: HashMap<i32, Option<Rc<RefCell<TreeNode>>>> = HashMap::new();
        let mut fa: HashMap<i32, i32> = HashMap::new();
        let mut s: Option<Rc<RefCell<TreeNode>>> = None;
        let mut t: Option<Rc<RefCell<TreeNode>>> = None;
        fn dfs(
            root: &Option<Rc<RefCell<TreeNode>>>,
            start_value: i32,
            dest_value: i32,
            s: &mut Option<Rc<RefCell<TreeNode>>>,
            t: &mut Option<Rc<RefCell<TreeNode>>>,
            m: &mut HashMap<i32, Option<Rc<RefCell<TreeNode>>>>,
            fa: &mut HashMap<i32, i32>,
        ) {
            if let Some(node) = root {
                let curr = node.borrow();
                if curr.val == start_value {
                    *s = root.clone();
                }
                if curr.val == dest_value {
                    *t = root.clone();
                }
                m.entry(curr.val).or_insert(root.clone());
                if curr.left.is_some() {
                    fa.entry(curr.left.as_ref().unwrap().borrow().val)
                        .or_insert(curr.val);
                    dfs(&curr.left, start_value, dest_value, s, t, m, fa);
                }
                if curr.right.is_some() {
                    fa.entry(curr.right.as_ref().unwrap().borrow().val)
                        .or_insert(curr.val);
                    dfs(&curr.right, start_value, dest_value, s, t, m, fa);
                }
            }
        }
        dfs(
            &root,
            start_value,
            dest_value,
            &mut s,
            &mut t,
            &mut m,
            &mut fa,
        );
        let path = |curr: &Option<Rc<RefCell<TreeNode>>>| -> String {
            let mut ans = String::new();
            let mut p = curr;
            while p != &root {
                let parv = *fa.get(&p.as_ref().unwrap().borrow().val).unwrap_or(&0);
                let d: &Option<Rc<RefCell<TreeNode>>> = &None;
                let par = m.get(&parv).unwrap_or(&d);
                if par.is_some()
                    && par.as_ref().unwrap().borrow().left.is_some()
                    && &par.as_ref().unwrap().borrow().left == p
                {
                    ans.push('L');
                } else {
                    ans.push('R');
                }
                p = par;
            }
            ans = ans.chars().rev().collect();
            ans
        };
        let path1 = path(&s);
        let path2 = path(&t);
        let (l1, l2) = (path1.len(), path2.len());
        let (p1, p2) = (path1.as_bytes(), path2.as_bytes());
        let mut i = 0;
        while i < l1.min(l2) {
            if p1[i] != p2[i] {
                break;
            }
            i += 1;
        }
        let mut ans = vec![b'U'; l1 - i];
        ans.extend(&p2[i..]);
        String::from_utf8(ans).unwrap()
    }
}
// @lc code=end

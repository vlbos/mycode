// 536\. Construct Binary Tree from String
// =======================================

// You need to construct a binary tree from a string consisting of parenthesis and integers.

// The whole input represents a binary tree. It contains an integer followed by zero, one or two pairs of parenthesis.
// The integer represents the root's value and a pair of parenthesis contains a child binary tree with the same structure.

// You always start to construct the **left** child node of the parent first if it exists.

// **Example:**

// **Input:** "4(2(3)(1))(6(5))"
// **Output:** return the tree root node representing the following tree:

//        4
//      /   \\
//     2     6
//    / \\   /
//   3   1 5

// **Note:**

// 1.  There will only be `'('`, `')'`, `'-'` and `'0'` ~ `'9'` in the input string.
// 2.  An empty tree is represented by `""` instead of `"()"`.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [Facebook](https://leetcode.ca/tags/#Facebook)

// @lc code=start
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub  struct TreeNode {
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
use crate::solutions::util::tree::TreeNode;
use std::cell::RefCell;
// use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn str2tree(s: String) -> Option<Rc<RefCell<TreeNode>>> {
        // let chars = {
        //     let mut cs = vec!['('];
        //     cs.extend(s.chars());
        //     cs.push(')');
        //     cs
        // };
        // let mut working = vec![];
        // let mut i = 0;
        // while i < chars.len() {
        //     let ch = chars[i];
        //     if ch == '(' {
        //         let mut curr = VecDeque::new();
        //         let mut j = i + 1;
        //         loop {
        //             let ch = chars[j];
        //             if ch == ')' || ch == '(' {
        //                 break;
        //             }
        //             j += 1;
        //         }
        //         curr.push_back(Solution::to_num(&chars[(i + 1)..j]));
        //         working.push(curr);
        //         i = j;
        //     } else if ch == ')' {
        //         let mut curr = working.pop().unwrap();
        //         let new_node = curr.pop_front().unwrap().map(|p_ref| {
        //             {
        //                 let mut p_node = p_ref.borrow_mut();
        //                 p_node.left = curr.pop_front().unwrap_or(None);
        //                 p_node.right = curr.pop_front().unwrap_or(None);
        //             }
        //             p_ref
        //         });
        //         let len = working.len();
        //         if working.len() == 0 {
        //             working.push(VecDeque::new());
        //             working[len].push_back(new_node);
        //         } else {
        //             working[len - 1].push_back(new_node);
        //         }
        //         i += 1;
        //     }
        // }
        // working[0].pop_front().unwrap()
        if s.is_empty() {
            return None;
        }
        let found = s.find("(");
        let mut j = s.len();
        let val = if let Some(i) = found {
            j = i;
            &s[..i]
        } else {
            &s[..]
        }
        .parse::<i32>()
        .unwrap();
        let mut cur = TreeNode::new(val);
        if found.is_none() {
            return Some(Rc::new(RefCell::new(cur)));
        }
        let mut cnt = 0;
        let bs = s.as_bytes();
        let mut start = j;
        for i in start..bs.len() {
            if bs[i] == b'(' {
                cnt += 1;
            } else if bs[i] == b')' {
                cnt -= 1;
            }
            if cnt != 0 {
                continue;
            }
            if start == j {
                cur.left = Self::str2tree(s[start + 1..i].to_string());
                start = i + 1;
            } else {
                cur.right = Self::str2tree(s[start + 1..i].to_string());
            }
        }
        Some(Rc::new(RefCell::new(cur)))
    }

    // fn to_num(s: &[char]) -> Option<Rc<RefCell<TreeNode>>> {
    //     s.iter()
    //         .cloned()
    //         .collect::<String>()
    //         .parse::<i32>()
    //         .map(|i| Rc::new(RefCell::new(TreeNode::new(i))))
    //         .ok()
    // }
}
// @lc code=end

#[allow(dead_code)]
pub  struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    use crate::tree;

    #[test]
    fn test_str2tree_1() {
        let tree = tree![4, 2, 6, 3, 1, 5];
        assert_eq!(Solution::str2tree(String::from("4(2(3)(1))(6(5))")), tree);
    }
}

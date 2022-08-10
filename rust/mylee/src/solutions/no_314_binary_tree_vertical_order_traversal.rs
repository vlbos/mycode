// 314\. Binary Tree Vertical Order Traversal
// ==========================================

// Given a binary tree, return the _vertical order_ traversal of its nodes' values. (ie, from top to bottom, column by column).

// If two nodes are in the same row and column, the order should be from **left to right**.

// **Examples 1:**

// **Input:** `[3,9,20,null,null,15,7]`
//    3
//   /\\
//  /  \\
//  9  20
//     /\\
//    /  \\
//   15   7

// **Output:**

// \[
//   \[9\],
//   \[3,15\],
//   \[20\],
//   \[7\]
// \]

// **Examples 2:**

// **Input:** `[3,9,8,4,0,1,7]

// `     3
//     /\\
//    /  \\
//    9   8
//   /\\  /\\
//  /  \\/  \\
//  4  01   7

// **Output:**

// \[
//   \[4\],
//   \[9\],
//   \[3,0,1\],
//   \[8\],
//   \[7\]
// \]

// **Examples 3:**

// **Input:** `[3,9,8,4,0,1,7,null,null,null,2,5]` (0's right child is 2 and 1's left child is 5)

//      3
//     /\\
//    /  \\
//    9   8
//   /\\  /\\
//  /  \\/  \\
//  4  01   7
//     /\\
//    /  \\
//    5   2

// **Output:**

// \[
//   \[4\],
//   \[9,5\],
//   \[3,0,1\],
//   \[8,2\],
//   \[7\]
// \]

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Adobe](https://leetcode.ca/tags/#Adobe) [Amazon](https://leetcode.ca/tags/#Amazon) [Bloomberg](https://leetcode.ca/tags/#Bloomberg) [ByteDance](https://leetcode.ca/tags/#ByteDance) [Databricks](https://leetcode.ca/tags/#Databricks) [Expedia](https://leetcode.ca/tags/#Expedia) [Facebook](https://leetcode.ca/tags/#Facebook) [Google](https://leetcode.ca/tags/#Google) [Mathworks](https://leetcode.ca/tags/#Mathworks) [Microsoft](https://leetcode.ca/tags/#Microsoft) [Oracle](https://leetcode.ca/tags/#Oracle) [Snapchat](https://leetcode.ca/tags/#Snapchat)

// @lc code=start
use crate::solutions::util::tree::TreeNode;
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
// use std::collections::{hash_map::HashMap, VecDeque};
use std::rc::Rc;

impl Solution {
    pub fn vertical_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        //     let mut levels: HashMap<i32, Vec<i32>> = HashMap::new();
        //     let mut queue = VecDeque::new();
        //     queue.push_back((0, root));
        //     while let Some((level, node_op)) = queue.pop_front() {
        //         if let Some(node_ref) = node_op {
        //             let mut node = node_ref.borrow_mut();
        //             let value = node.val;
        //             levels
        //                 .entry(level)
        //                 .and_modify(|arr| {
        //                     arr.push(value);
        //                 })
        //                 .or_insert(vec![value]);
        //             queue.push_back((level - 1, node.left.take()));
        //             queue.push_back((level + 1, node.right.take()));
        //         }
        //     }
        //     let mut ret = levels.into_iter().collect::<Vec<(i32, Vec<i32>)>>();
        //     ret.sort_by_key(|(k, _)| *k);
        //     ret.into_iter().map(|(_, v)| v).collect()
        use std::collections::BTreeMap;
        use std::collections::BTreeSet;
        fn dfs(
            root: &Option<Rc<RefCell<TreeNode>>>,
            i: i32,
            j: i32,
            ans: &mut BTreeMap<i32, BTreeSet<Vec<i32>>>,
        ) {
            if let Some(node) = root {
                let node = node.borrow();
                ans.entry(i)
                    .or_insert(BTreeSet::new())
                    .insert(vec![j, node.val]);
                dfs(&node.left, i - 1, j + 1, ans);
                dfs(&node.right, i + 1, j + 1, ans);
            }
        }
        let mut ans = BTreeMap::new();
        dfs(&root, 0, 0, &mut ans);
        ans.values()
            .map(|x| x.iter().map(|b| b[1]).collect::<Vec<i32>>())
            .collect()
    }
}
// @lc code=end
 
#[allow(dead_code)] 
 struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    use crate::tree;

    #[test]
    fn test_binary_tree_vertical_order_traversal_1() {
        let tree = tree![3, 9, 20, null, null, 15, 7];
        assert_eq!(
            Solution::vertical_order(tree),
            vec![vec![9], vec![3, 15], vec![20], vec![7]]
        );
    }

    #[test]
    fn test_binary_tree_vertical_order_traversal_2() {
        let tree = tree![3, 9, 8, 4, 0, 1, 7];
        assert_eq!(
            Solution::vertical_order(tree),
            vec![vec![4], vec![9], vec![3, 0, 1], vec![8], vec![7]]
        );
    }

    #[test]
    fn test_binary_tree_vertical_order_traversal_3() {
        let tree = tree![3, 9, 8, 4, 0, 1, 7, null, null, null, 2, 5];
        assert_eq!(
            Solution::vertical_order(tree),
            vec![vec![4], vec![9, 5], vec![3, 0, 1], vec![8, 2], vec![7]]
        );
    }

    #[test]
    fn test_binary_tree_vertical_order_traversal_4() {
        let tree = None;
        assert_eq!(Solution::vertical_order(tree), Vec::<Vec<i32>>::new());
    }
}

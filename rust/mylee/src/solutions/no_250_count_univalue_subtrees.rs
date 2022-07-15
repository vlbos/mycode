// 250\. Count Univalue Subtrees
// =============================

// Given a binary tree, count the number of uni-value subtrees.

// A Uni-value subtree means all nodes of the subtree have the same value.

// **Example :**

// **Input:**  root = \[5,1,5,5,5,null,5\]

//               5
//              / \\
//             1   5
//            / \\   \\
//           5   5   5

// **Output:** 4

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [Bloomberg](https://leetcode.ca/tags/#Bloomberg) [Box](https://leetcode.ca/tags/#Box) [Facebook](https://leetcode.ca/tags/#Facebook) [Google](https://leetcode.ca/tags/#Google)
use crate::solutions::util::tree::TreeNode;
// @lc code=start
use std::cell::RefCell;
use std::rc::Rc;

// enum TreeState {
//     Uni((usize, i32)),
//     Diff(usize),
//     Empty,
// }

impl Solution {
    pub fn count_unival_subtrees(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // use TreeState::{Diff, Empty, Uni};
        // let res = Solution::count_unival_subtrees_rec(&root);
        // (match res {
        //     Diff(count) => count,
        //     Uni((count, _)) => count,
        //     Empty => 0,
        // }) as i32
        use std::collections::HashSet;
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, ans: &mut i32) -> HashSet<i32> {
            if let Some(p) = root {
                let node = p.borrow();
                let mut values = HashSet::from([node.val]);
                let l = dfs(&node.left, ans);
                let r = dfs(&node.right, ans);
                values = values
                    .union(&l)
                    .cloned()
                    .collect::<HashSet<i32>>()
                    .union(&r)
                    .cloned()
                    .collect();
                if values.len() == 1 {
                    *ans += 1;
                }
                values
            } else {
                HashSet::new()
            }
        }
        let mut ans = 0;
        dfs(&root, &mut ans);
        ans
    }

    // fn count_unival_subtrees_rec(root: &Option<Rc<RefCell<TreeNode>>>) -> TreeState {
    //     use TreeState::{Diff, Empty, Uni};
    //     match root {
    //         Some(node_ref) => {
    //             let node_b = node_ref.borrow();
    //             let left = Solution::count_unival_subtrees_rec(&node_b.left);
    //             let right = Solution::count_unival_subtrees_rec(&node_b.right);
    //             let mut is_uni = true;
    //             let mut uni_count = 0;
    //             match left {
    //                 Uni((count, value)) => {
    //                     uni_count += count;
    //                     if value != node_b.val {
    //                         is_uni = false;
    //                     }
    //                 }
    //                 Diff(count) => {
    //                     uni_count += count;
    //                     is_uni = false;
    //                 }
    //                 Empty => {}
    //             }
    //             match right {
    //                 Uni((count, value)) => {
    //                     uni_count += count;
    //                     if value != node_b.val {
    //                         is_uni = false;
    //                     }
    //                 }
    //                 Diff(count) => {
    //                     uni_count += count;
    //                     is_uni = false;
    //                 }
    //                 Empty => {}
    //             }
    //             if is_uni {
    //                 uni_count += 1;
    //                 Uni((uni_count, node_b.val))
    //             } else {
    //                 Diff(uni_count)
    //             }
    //         }
    //         None => Empty,
    //     }
    // }
}
// @lc code=end
struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    use crate::tree;

    #[test]
    fn test_count_unival_subtrees() {
        let tree = tree![5, 1, 5, 5, 5, null, 5];
        assert_eq!(Solution::count_unival_subtrees(tree), 4);
    }
}

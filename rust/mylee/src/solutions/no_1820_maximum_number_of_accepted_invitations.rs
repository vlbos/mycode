// 1820\. Maximum Number of Accepted Invitations
// =============================================

// There are `m` boys and `n` girls in a class attending an upcoming party.

// You are given an `m x n` integer matrix `grid`, where `grid[i][j]` equals `0` or `1`. If `grid[i][j] == 1`, then that means the `ith` boy can invite the `jth` girl to the party. A boy can invite at most **one girl**, and a girl can accept at most **one invitation** from a boy.

// Return _the **maximum** possible number of accepted invitations._

// **Example 1:**

// **Input:** grid = \[\[1,1,1\],
//                \[1,0,1\],
//                \[0,0,1\]\]
// **Output:** 3 **Explanation:** The invitations are sent as follows:
// - The 1st boy invites the 2nd girl.
// - The 2nd boy invites the 1st girl.
// - The 3rd boy invites the 3rd girl.

// **Example 2:**

// **Input:** grid = \[\[1,0,1,0\],
//                \[1,0,0,0\],
//                \[0,0,1,0\],
//                \[1,1,1,0\]\]
// **Output:** 3
// **Explanation:** The invitations are sent as follows:
// -The 1st boy invites the 3rd girl.
// -The 2nd boy invites the 1st girl.
// -The 3rd boy invites no one.
// -The 4th boy invites the 2nd girl.

// **Constraints:**

// *   `grid.length == m`
// *   `grid[i].length == n`
// *   `1 <= m, n <= 200`
// *   `grid[i][j]` is either `0` or `1`.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Unknown](https://leetcode.ca/tags/#Unknown)

use super::util::tree::TreeNode;

#[allow(dead_code)]
pub struct Solution {}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn check_equivalence(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        use std::collections::HashMap;
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, v: i32, freq: &mut HashMap<i32, i32>) {
            if root.is_none() {
                return;
            }
            let node = root.as_ref().unwrap().borrow();
            if (node.val as u8 as char).is_ascii_alphabetic() {
                *freq.entry(node.val).or_insert(0) += v;
            } else {
                dfs(&node.left, v, freq);
                dfs(&node.right, v, freq);
            }
        }
        let mut freq = HashMap::new();
        dfs(&root1, 1, &mut freq);
        dfs(&root2, -1, &mut freq);
        if freq.values().any(|v| *v > 0) {
            false
        } else {
            true
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    // use crate::tree;
    use super::super::util::tree::to_tree;
    fn to_exp_tree(s: &str) -> Option<Rc<RefCell<TreeNode>>> {
        to_tree(
            s.split(',')
                .map(|x| {
                    if x == "null" {
                        None
                    } else {
                        Some(x.as_bytes()[0] as i32)
                    }
                })
                .collect::<Vec<Option<i32>>>(),
        )
    }
    #[test]
    pub fn test_check_equivalence_1() {
        assert!(Solution::check_equivalence(
            to_exp_tree("x"),
            to_exp_tree("x")
        ));
    }
    #[test]
    pub fn test_check_equivalence_2() {
        assert!(Solution::check_equivalence(
            to_exp_tree("+,a,+,null,null,b,c"),
            to_exp_tree("+,+,b,c,a")
        ));
    }
    #[test]
    pub fn test_check_equivalence_3() {
        assert!(!Solution::check_equivalence(
            to_exp_tree("+,a,+,null,null,b,c"),
            to_exp_tree("+,+,b,d,a")
        ));
    }
}

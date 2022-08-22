// 1714\. Sum Of Special Evenly-Spaced Elements In Array
// =====================================================

// You are given a **0-indexed** integer array `nums` consisting of `n` non-negative integers.

// You are also given an array `queries`, where `queries[i] = [xi, yi]`. The answer to the `ith` query is the sum of all `nums[j]` where `xi <= j < n` and `(j - xi)` is divisible by `yi`.

// Return _an array_ `answer` _where_ `answer.length == queries.length` _and_ `answer[i]` _is the answer to the_ `ith` _query **modulo**_ `109 + 7`.

// **Example 1:**

// **Input:** nums = \[0,1,2,3,4,5,6,7\], queries = \[\[0,3\],\[5,1\],\[4,2\]\]
// **Output:** \[9,18,10\]
// **Explanation:** The answers of the queries are as follows:
// 1) The j indices that satisfy this query are 0, 3, and 6. nums\[0\] + nums\[3\] + nums\[6\] = 9
// 2) The j indices that satisfy this query are 5, 6, and 7. nums\[5\] + nums\[6\] + nums\[7\] = 18
// 3) The j indices that satisfy this query are 4 and 6. nums\[4\] + nums\[6\] = 10

// **Example 2:**

// **Input:** nums = \[100,200,101,201,102,202,103,203\], queries = \[\[0,7\]\]
// **Output:** \[303\]

// **Constraints:**

// *   `n == nums.length`
// *   `1 <= n <= 5 * 104`
// *   `0 <= nums[i] <= 109`
// *   `1 <= queries.length <= 1.5 * 105`
// *   `0 <= xi < n`
// *   `1 <= yi <= 5 * 104`

// ### Difficulty:

// Hard

// ### Lock:

// Prime

// ### Company:

// [MakeMyTrip](https://leetcode.ca/tags/#MakeMyTrip) [Sprinklr](https://leetcode.ca/tags/#Sprinklr)

// int[] solve(int[] nums, int[][] queries)

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

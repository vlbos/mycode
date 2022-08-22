// 1724\. Checking Existence of Edge Length Limited Paths II
// =========================================================

// An undirected graph of `n` nodes is defined by `edgeList`, where `edgeList[i] = [ui, vi, disi]` denotes an edge between nodes `ui` and `vi` with distance `disi`. Note that there may be **multiple** edges between two nodes, and the graph may not be connected.

// Implement the `DistanceLimitedPathsExist` class:

// *   `DistanceLimitedPathsExist(int n, int[][] edgeList)` Initializes the class with an undirected graph.
// *   `boolean query(int p, int q, int limit)` Returns `true` if there exists a path from `p` to `q` such that each edge on the path has a distance **strictly less than** `limit`, and otherwise `false`.

// **Example 1:**

// **![](https://assets.leetcode.com/uploads/2021/01/05/messed.png)**

// **Input**
// \["DistanceLimitedPathsExist", "query", "query", "query", "query"\]
// \[\[6, \[\[0, 2, 4\], \[0, 3, 2\], \[1, 2, 3\], \[2, 3, 1\], \[4, 5, 5\]\]\], \[2, 3, 2\], \[1, 3, 3\], \[2, 0, 3\], \[0, 5, 6\]\]
// **Output**
// \[null, true, false, true, false\]

// **Explanation**
// DistanceLimitedPathsExist distanceLimitedPathsExist = new DistanceLimitedPathsExist(6, \[\[0, 2, 4\], \[0, 3, 2\], \[1, 2, 3\], \[2, 3, 1\], \[4, 5, 5\]\]);
// distanceLimitedPathsExist.query(2, 3, 2); // return true. There is an edge from 2 to 3 of distance 1, which is less than 2.
// distanceLimitedPathsExist.query(1, 3, 3); // return false. There is no way to go from 1 to 3 with distances **strictly** less than 3.
// distanceLimitedPathsExist.query(2, 0, 3); // return true. There is a way to go from 2 to 0 with distance < 3: travel from 2 to 3 to 0.
// distanceLimitedPathsExist.query(0, 5, 6); // return false. There are no paths from 0 to 5.

// `**Constraints:**`

// *   `2 <= n <= 104`
// *   `0 <= edgeList.length <= 104`
// *   `edgeList[i].length == 3`
// *   `0 <= ui, vi, p, q <= n-1`
// *   `ui != vi`
// *   `p != q`
// *   `1 <= disi, limit <= 109`
// *   At most `104` calls will be made to `query`.

// ### Difficulty:

// Hard

// ### Lock:

// Prime

// ### Company:

// [Google](https://leetcode.ca/tags/#Google)

// public DistanceLimitedPathsExist(int n, int[][] edgeList) {

//     public boolean query(int p, int q, int limit)

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

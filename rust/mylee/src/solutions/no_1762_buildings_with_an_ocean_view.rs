// 1762\. Buildings With an Ocean View
// ===================================

// There are `n` buildings in a line. You are given an integer array `heights` of size `n` that represents the heights of the buildings in the line.

// The ocean is to the right of the buildings. A building has an ocean view if the building can see the ocean without obstructions. Formally, a building has an ocean view if all the buildings to its right have a **smaller** height.

// Return a list of indices **(0-indexed)** of buildings that have an ocean view, sorted in increasing order.

// **Example 1:**

// **Input:** heights = \[4,2,3,1\]
// **Output:** \[0,2,3\]
// **Explanation:** Building 1 (0-indexed) does not have an ocean view because building 2 is taller.

// **Example 2:**

// **Input:** heights = \[4,3,2,1\]
// **Output:** \[0,1,2,3\]
// **Explanation:** All the buildings have an ocean view.

// **Example 3:**

// **Input:** heights = \[1,3,2,4\]
// **Output:** \[3\]
// **Explanation:** Only building 3 has an ocean view.

// **Example 4:**

// **Input:** heights = \[2,2,2,2\]
// **Output:** \[3\]
// **Explanation:** Buildings cannot see the ocean if there are buildings of the **same** height to its right.

// **Constraints:**

// *   `1 <= heights.length <= 105`
// *   `1 <= heights[i] <= 109`

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Facebook](https://leetcode.ca/tags/#Facebook)

// int[] findBuildings(int[] heights)

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

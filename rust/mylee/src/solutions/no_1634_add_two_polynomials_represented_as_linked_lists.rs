// 1634\. Add Two Polynomials Represented as Linked Lists
// ======================================================

// A polynomial linked list is a special type of linked list where every node represents a term in a polynomial expression.

// Each node has three attributes:

// *   `coefficient`: an integer representing the number multiplier of the term. The coefficient of the term `**9**x4` is `9`.
// *   `power`: an integer representing the exponent. The power of the term `9x**4**` is `4`.
// *   `next`: a pointer to the next node in the list, or `null` if it is the last node of the list.

// For example, the polynomial `5x3 + 4x - 7` is represented by the polynomial linked list illustrated below:

// ![](https://assets.leetcode.com/uploads/2020/09/30/polynomial2.png)

// The polynomial linked list must be in its standard form: the polynomial must be in **strictly** descending order by its `power` value.
// Also, terms with a `coefficient` of `0` are omitted.

// Given two polynomial linked list heads, `poly1` and `poly2`, add the polynomials together and return _the head of the sum of the polynomials_.

// **`PolyNode` format:**

// The input/output format is as a list of `n` nodes, where each node is represented as its `[coefficient, power]`.
// For example, the polynomial `5x3 + 4x - 7` would be represented as: `[[5,3],[4,1],[-7,0]]`.

// **Example 1:**

// ![](https://assets.leetcode.com/uploads/2020/10/14/ex1.png)

// **Input:** poly1 = \[\[1,1\]\], poly2 = \[\[1,0\]\]
// **Output:** \[\[1,1\],\[1,0\]\]
// **Explanation:** poly1 = x. poly2 = 1. The sum is x + 1.

// **Example 2:**

// **Input:** poly1 = \[\[2,2\],\[4,1\],\[3,0\]\], poly2 = \[\[3,2\],\[-4,1\],\[-1,0\]\]
// **Output:** \[\[5,2\],\[2,0\]\]
// **Explanation:** poly1 = 2x2 + 4x + 3. poly2 = 3x2 - 4x - 1. The sum is 5x2 + 2. Notice that we omit the "0x" term.

// **Example 3:**

// **Input:** poly1 = \[\[1,2\]\], poly2 = \[\[-1,2\]\]
// **Output:** \[\]
// **Explanation:** The sum is 0. We return an empty list.

// **Constraints:**

// *   `0 <= n <= 104`
// *   `-109 <= PolyNode.coefficient <= 109`
// *   `PolyNode.coefficient != 0`
// *   `0 <= PolyNode.power <= 109`
// *   `PolyNode.power > PolyNode.next.power`

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon)

//  PolyNode addPoly(PolyNode poly1, PolyNode poly2)

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

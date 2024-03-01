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

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PolyNode {
    pub coefficient: i32,
    pub power: i32,
    pub next: Option<Box<PolyNode>>,
}

impl PolyNode {
    #[inline]
    pub fn new(coefficient: i32, power: i32) -> Self {
        Self {
            coefficient,
            power,
            next: None,
        }
    }
}

// helper function for test
pub fn to_poly_list(vec: Vec<Vec<i32>>) -> Option<Box<PolyNode>> {
    let mut current = None;
    for v in vec.iter().rev() {
        let mut node = PolyNode::new(v[0], v[1]);
        node.next = current;
        current = Some(Box::new(node));
    }
    current
}

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn add_poly(
        poly1: Option<Box<PolyNode>>,
        poly2: Option<Box<PolyNode>>,
    ) -> Option<Box<PolyNode>> {
        let (mut p1, mut p2) = (poly1, poly2);
        let mut ans = None;
        let mut cur = &mut ans;
        while p1.is_some() || p2.is_some() {
            if p1.is_none()
                || (p2.is_some() && p2.as_ref().unwrap().power > p1.as_ref().unwrap().power)
            {
                let next = p2.as_mut().unwrap().next.take();
                cur = &mut cur.get_or_insert(p2.unwrap()).next;
                p2 = next;
            } else if p2.is_none()
                || (p1.is_some() && p1.as_ref().unwrap().power > p2.as_ref().unwrap().power)
            {
                let next = p1.as_mut().unwrap().next.take();
                cur = &mut cur.get_or_insert(p1.unwrap()).next;
                p1 = next;
            } else {
                let val = p1.as_ref().unwrap().coefficient + p2.as_ref().unwrap().coefficient;
                if val != 0 {
                    cur = &mut cur
                        .get_or_insert(Box::new(PolyNode {
                            coefficient: val,
                            power: p1.as_ref().unwrap().power,
                            next: None,
                        }))
                        .next;
                }
                p1 = p1.as_mut().unwrap().next.take();
                p2 = p2.as_mut().unwrap().next.take();
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_add_poly_1() {
        assert_eq!(
            to_poly_list(vec![vec![1, 1], vec![1, 0]]),
            Solution::add_poly(
                to_poly_list(vec![vec![1, 1]]),
                to_poly_list(vec![vec![1, 0]])
            )
        );
    }
    #[test]
    pub fn test_add_poly_2() {
        assert_eq!(
            to_poly_list(vec![vec![5, 2], vec![2, 0]]),
            Solution::add_poly(
                to_poly_list(vec![vec![2, 2], vec![4, 1], vec![3, 0]]),
                to_poly_list(vec![vec![3, 2], vec![-4, 1], vec![-1, 0]])
            )
        );
    }
    #[test]
    pub fn test_add_poly_3() {
        assert_eq!(
            to_poly_list(vec![]),
            Solution::add_poly(
                to_poly_list(vec![vec![1, 2]]),
                to_poly_list(vec![vec![-1, 2]])
            )
        );
    }
}

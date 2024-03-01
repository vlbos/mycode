// [2046\. Sort Linked List Already Sorted Using Absolute Values (Medium)](https://leetcode.com/problems/sort-linked-list-already-sorted-using-absolute-values/)[](https://leetcode.ca/2021-10-30-2046-Sort-Linked-List-Already-Sorted-Using-Absolute-Values/#2046-sort-linked-list-already-sorted-using-absolute-values-medium)
// =============================================================================================================================================================================================================================================================================================================================

// Given the `head` of a singly linked list that is sorted in **non-decreasing** order using the **absolute values** of its nodes,
// return _the list sorted in **non-decreasing** order using the **actual values** of its nodes_.

// **Example 1:**

// ![](https://assets.leetcode.com/uploads/2021/10/17/image-20211017201240-3.png)

// **Input:** head = \[0,2,-5,5,10,-10\]
// **Output:** \[-10,-5,0,2,5,10\]
// **Explanation:**
// The list sorted in non-descending order using the absolute values of the nodes is \[0,2,-5,5,10,-10\].
// The list sorted in non-descending order using the actual values is \[-10,-5,0,2,5,10\].

// **Example 2:**

// ![](https://assets.leetcode.com/uploads/2021/10/17/image-20211017201318-4.png)

// **Input:** head = \[0,1,2\]
// **Output:** \[0,1,2\]
// **Explanation:**
// The linked list is already sorted in non-decreasing order.

// **Example 3:**

// **Input:** head = \[1\]
// **Output:** \[1\]
// **Explanation:**
// The linked list is already sorted in non-decreasing order.

// **Constraints:**

// *   The number of nodes in the list is the range `[1, 105]`.
// *   `-5000 <= Node.val <= 5000`
// *   `head` is sorted in non-decreasing order using the absolute value of its nodes.

// **Follow up:**

// *   Can you think of a solution with `O(n)` time complexity?

// **Related Topics**:
// [Linked List](https://leetcode.com/tag/linked-list/), [Two Pointers](https://leetcode.com/tag/two-pointers/), [Sorting](https://leetcode.com/tag/sorting/)

// **Similar Questions**:

// *   [Sort List (Medium)](https://leetcode.com/problems/sort-list/)

// Solution 1.[](https://leetcode.ca/2021-10-30-2046-Sort-Linked-List-Already-Sorted-Using-Absolute-Values/#solution-1)
// --------------------------------------------------------------------------------------------------------------------

//     // OJ: https://leetcode.com/problems/sort-linked-list-already-sorted-using-absolute-values/
//     // Time: O(N)
//     // Space: O(1)
//     class Solution {
//     public:
//         ListNode* sort_linked_list(ListNode* head) {
use super::util::linked_list::ListNode;
#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn sort_linked_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut p = head;
        let mut positive = None;
        let mut target = &mut positive;
        let mut negative = None;
        while let Some(mut node) = p {
            p = node.next.take();
            if node.val < 0 {
                node.next = negative;
                negative = Some(node);
            } else {
                target = &mut target.get_or_insert(node).next;
            }
        }
        if negative.is_some() {
            let mut target = &mut negative;
            while target.is_some() && target.as_ref().unwrap().next.is_some() {
                if let Some(node) = target {
                    target = &mut node.next;
                }
            }

            target.as_mut().unwrap().next = positive;
            negative
        } else {
            positive
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::linked;
    #[test]
    pub fn test_sort_linked_list_1() {
        assert_eq!(
            linked![-10, -5, 0, 2, 5, 10],
            Solution::sort_linked_list(linked![0, 2, -5, 5, 10, -10])
        );
    }
    #[test]
    pub fn test_sort_linked_list_2() {
        assert_eq!(
            linked![0, 1, 2],
            Solution::sort_linked_list(linked![0, 1, 2])
        );
    }
    #[test]
    pub fn test_sort_linked_list_3() {
        assert_eq!(linked![1], Solution::sort_linked_list(linked![1]));
    }
}

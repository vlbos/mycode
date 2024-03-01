// 1836\. Remove Duplicates From an Unsorted Linked List
// =====================================================

// Given the `head` of a linked list, find all the values that appear **more than once** in the list and delete the nodes that have any of those values.

// Return _the linked list after the deletions._

// **Example 1:**

// ![](https://assets.leetcode.com/uploads/2021/04/21/tmp-linked-list.jpg)

// **Input:** head = \[1,2,3,2\]
// **Output:** \[1,3\]
// **Explanation:** 2 appears twice in the linked list, so all 2's should be deleted. After deleting all 2's, we are left with \[1,3\].

// **Example 2:**

// ![](https://assets.leetcode.com/uploads/2021/04/21/tmp-linked-list-1.jpg)

// **Input:** head = \[2,1,1,2\]
// **Output:** \[\]
// **Explanation:** 2 and 1 both appear twice. All the elements should be deleted.

// **Example 3:**

// ![](https://assets.leetcode.com/uploads/2021/04/21/tmp-linked-list-2.jpg)

// **Input:** head = \[3,2,2,1,3,2,4\]
// **Output:** \[1,4\]
// **Explanation:** 3 appears twice and 2 appears three times. After deleting all 3's and 2's, we are left with \[1,4\].

// **Constraints:**

// *   The number of nodes in the list is in the range `[1, 105]`
// *   `1 <= Node.val <= 105`

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Microsoft](https://leetcode.ca/tags/#Microsoft)

use super::util::linked_list::ListNode;

//  ListNode delete_duplicates_unsorted(ListNode head) {

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn delete_duplicates_unsorted(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut cnt = std::collections::HashMap::new();
        let mut p = &head;
        while let Some(next) = p {
            *cnt.entry(next.val).or_insert(0) += 1;
            p = &next.next;
        }
        let mut p = head;
        let mut ans = None;
        let mut target = &mut ans;
        while let Some(mut node) = p {
            p = node.next.take();
            if *cnt.get(&node.val).unwrap() == 1 {
                target = &mut target.get_or_insert(node).next;
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::linked;

    #[test]
    pub fn test_delete_duplicates_unsorted_1() {
        assert_eq!(
            linked![1, 3],
            Solution::delete_duplicates_unsorted(linked![1, 2, 3, 2],)
        );
    }
    #[test]
    pub fn test_delete_duplicates_unsorted_2() {
        assert_eq!(
            linked![],
            Solution::delete_duplicates_unsorted(linked![2, 1, 1, 2],)
        );
    }
    #[test]
    pub fn test_delete_duplicates_unsorted_3() {
        assert_eq!(
            linked![1, 4],
            Solution::delete_duplicates_unsorted(linked![3, 2, 2, 1, 3, 2, 4],)
        );
    }
}

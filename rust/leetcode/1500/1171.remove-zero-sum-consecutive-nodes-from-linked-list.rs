/*
 * @lc app=leetcode id=1171 lang=rust
 *
 * [1171] Remove Zero Sum Consecutive Nodes from Linked List
 */

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn remove_zero_sum_sublists(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut m = std::collections::HashMap::new();
        let mut sum = 0;
        let mut p = &head;
        while let Some(n) = p {
            sum += n.val;
            m.insert(sum, Some(n.clone()));
            p = &n.next;
        }
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut p = &mut dummy;
        sum = 0;
        while let Some(n) = p {
            sum += n.val;
            if let Some(mut node) = m.get_mut(&sum) {
                    n.next = node.as_mut().unwrap().next.take();
            }
            p = &mut n.next;
        }
        dummy.unwrap().next
    }
}
// @lc code=end

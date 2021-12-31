/*
 * @lc app=leetcode id=92 lang=rust
 *
 * [92] Reverse Linked List II
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
    pub fn reverse_between(head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
        if left==right{
        return head;
        }
        let mut stack = vec![];
        let mut next = head;
        while let Some(this) = next {
            stack.push(this.val);
            next = this.next;
        }
        stack[left as usize - 1..=right as usize - 1].reverse();
        let mut next = None;
        while let Some(val) = stack.pop() {
            next = Some(Box::new(ListNode { next, val }));
        }
        next

    }
}
// @lc code=end


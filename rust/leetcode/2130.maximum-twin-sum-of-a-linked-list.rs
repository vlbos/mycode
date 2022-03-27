/*
 * @lc app=leetcode id=2130 lang=rust
 *
 * [2130] Maximum Twin Sum of a Linked List
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
    pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
        let mut slow = &head;
        let mut fast = &head;
        let mut ans = 0;
        let mut pre=Vec::new();
        while fast.is_some() && fast.as_ref().unwrap().next.is_some(){
            fast=&fast.as_ref().unwrap().next.as_ref().unwrap().next;
            pre.push(slow.as_ref().unwrap().val);
            slow=&slow.as_ref().unwrap().next;
        }
        let mut i = pre.len()-1;
        while slow.is_some(){
            ans=ans.max(pre[i]+slow.as_ref().unwrap().val);
            slow=&slow.as_ref().unwrap().next;
            i-=1;
        }
        ans
    }
}
// @lc code=end

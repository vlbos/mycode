/*
 * @lc app=leetcode id=1290 lang=rust
 *
 * [1290] Convert Binary Number in a Linked List to Integer
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
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        let mut d =0;
        let mut p = &head;
        while let Some(n)=p{
            d*=2;
            d+=n.val;
            p = &n.next;
        }
        d
    }
}
// @lc code=end


/*
 * @lc app=leetcode id=1721 lang=rust
 *
 * [1721] Swapping Nodes in a Linked List
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
    pub fn swap_nodes(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut p = head.as_ref();
        let mut q = head.as_ref();

        for _ in 0..k-1 {
            q = q.unwrap().next.as_ref();
        }
        let kv = q.unwrap().val;
         println!("{},{}",kv,1);
        let mut i = 0;
        while q.as_ref().unwrap().next.is_some() {
            q = q.unwrap().next.as_ref();
            p = p.unwrap().next.as_ref();
            i += 1;
        }
        let rkv = p.unwrap().val;
        let mut head = head;
        let mut p = head.as_mut();
        let mut j = 0;
        while let Some(n)=p {
            if j == k-1 {
                n.val = rkv;
            } else if j == i {
                n.val = kv;
            }
            p = n.next.as_mut();
            j += 1;
        }
        head
    }
}
// @lc code=end

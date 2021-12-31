/*
 * @lc app=leetcode id=817 lang=rust
 *
 * [817] Linked List Components
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
    pub fn num_components(head: Option<Box<ListNode>>, nums: Vec<i32>) -> i32 {
        let mut p = &head;
        let ns = nums.iter().collect::<std::collections::HashSet<_>>();
        let mut ans = 0;
        while let Some(q) = p {
            if ns.contains(&q.val) {
                if let Some(next) = &q.next {
                    if !ns.contains(&next.val) {
                        ans += 1;
                    }
                } else {
                    ans += 1;
                }
            }
            p = &q.next;
        }
        ans
    }
}
// @lc code=end

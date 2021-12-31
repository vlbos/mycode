/*
 * @lc app=leetcode id=147 lang=rust
 *
 * [147] Insertion Sort List
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
    pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut ans : Option<Box<ListNode>> = None;
        let mut tail = &mut ans;
        while let Some(mut n)=head.take(){
             head = n.next.take();
             tail = &mut ans;
             loop {
                    if let Some(o)=tail{
                        if o.val<n.val{
                            tail = &mut o.next;
                        }else{
                            o.next = Some(std::mem::replace(o, n));

                            break;
                        }
                    }else{
                        *tail=Some(n);
                        break;
                    }
             }
        }
        ans
    }
}
// @lc code=end


/*
 * @lc app=leetcode.cn id=206 lang=rust
 *
 * [206] 反转链表
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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // let mut prev: Option<Box<ListNode>> = None;
        // let mut curr = head;
        
        // while let Some(mut boxed_node) = curr {
        //     curr = boxed_node.next.take().take();
        //     // let mut next = boxed_node.next.take();
        //     boxed_node.next = prev.take();
        //     prev = Some(boxed_node);
        //     // curr = next.take();
        // }
        
        // prev

        fn inner_reverse_list(reversed:Option<Box<ListNode>>,head: Option<Box<ListNode>>) -> Option<Box<ListNode>>{
                    if let Some(mut _head)=head{
                        let new_head=std::mem::replace(&mut _head.next,reversed);
                        inner_reverse_list(Some(_head),new_head)
                    }
                    else{
                        reversed
                    }
        }
        inner_reverse_list(None,head)
    }
}
// @lc code=end


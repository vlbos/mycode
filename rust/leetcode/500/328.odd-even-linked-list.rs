/*
 * @lc app=leetcode id=328 lang=rust
 *
 * [328] Odd Even Linked List
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
    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_some(){
            let mut node = head;
            let mut odd :Option<Box<ListNode>>= None;
            let mut odd_tail = &mut odd;
            let mut even:Option<Box<ListNode>>  = None;
            let mut even_tail= &mut even;
            let mut flag = true;
            while let Some(mut n)=node.take(){
                    node=n.next.take();
                    if flag{
                        odd_tail = &mut odd_tail.get_or_insert(n).next;
                    }else{
                        even_tail = &mut even_tail.get_or_insert(n).next;
                    }
                    flag =!flag;
            }
       
            *odd_tail=even;
           
            return odd;
        }
        None
    }
}
// @lc code=end


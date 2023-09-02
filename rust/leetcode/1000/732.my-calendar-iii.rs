/*
 * @lc app=leetcode id=732 lang=rust
 *
 * [732] My Calendar III
 */

// @lc code=start
use std::collections::BTreeMap;
struct MyCalendarThree {
    delta: BTreeMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendarThree {
    fn new() -> Self {
        Self {
            delta: BTreeMap::new(),
        }
    }

    fn book(&mut self, start: i32, end: i32) -> i32 {
        self.delta.entry(start).and_modify(|x| *x += 1).or_insert(1);
        self.delta.entry(end).and_modify(|x| *x -= 1).or_insert(-1);
        let mut active = 0;
        let mut ans = 0;
        for &v in self.delta.values() {
            active += v;
            if active > ans {
                ans = active;
            }
        }
        ans
    }
}

/**
 * Your MyCalendarThree object will be instantiated and called as such:
 * let obj = MyCalendarThree::new();
 * let ret_1: i32 = obj.book(start, end);
 */
// @lc code=end
use std::collections::HashMap;
struct MyCalendarThree {
tree:HashMap<i32,i32>,
lazy:HashMap<i32,i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendarThree {

    fn new() -> Self {
        Self{tree:HashMap::new(),lazy:HashMap::new()}
    }
    
    fn book(&mut self, start_time: i32, end_time: i32) -> i32 {
       fn update(start_time: i32, end_time: i32,l:i32,r:i32,idx:i32,tree:&mut HashMap<i32,i32>,lazy:&mut HashMap<i32,i32>){
           if start_time>r|| end_time<l{
               return 
           }
           if start_time<=l && end_time>=r{
               *tree.entry(idx).or_insert(0)+=1;
               *lazy.entry(idx).or_insert(0)+=1;
               return 
           }
           let mid=(l+r)/2;
           update(start_time,end_time,l,mid,idx*2,tree,lazy);
           update(start_time,end_time,mid+1,r,idx*2+1,tree,lazy);
           let v=*lazy.get(&idx).unwrap_or(&0)+(*tree.get(&(idx*2)).unwrap_or(&0)).max(*tree.get(&(idx*2+1)).unwrap_or(&0));
           tree.insert(idx,v);
       }
       update(start_time,end_time-1,0,1_000_000_000,1,&mut self.tree,&mut self.lazy);
       self.tree[&1]
       
    }
}

/**
 * Your MyCalendarThree object will be instantiated and called as such:
 * let obj = MyCalendarThree::new();
 * let ret_1: i32 = obj.book(startTime, endTime);
 */
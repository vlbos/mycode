/*
 * @lc app=leetcode id=731 lang=rust
 *
 * [731] My Calendar II
 */

// @lc code=start
struct MyCalendarTwo {
delta:std::collections::BTreeMap<i32,i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendarTwo {

    fn new() -> Self {
        Self{delta:std::collections::BTreeMap::new()}
    }
    
    fn book(&mut self, start: i32, end: i32) -> bool {
        *self.delta.entry(start).or_insert(0)+=1;
        *self.delta.entry(end).or_insert(0)-=1;
        let mut active = 0;
        for v in self.delta.values(){
            active+=v;
            if active>=3{
                *self.delta.entry(start).or_insert(0)-=1;
                *self.delta.entry(end).or_insert(0)+=1;
                if *self.delta.get(&start).unwrap_or(&0)==0{
                self.delta.remove(&start);
                }
            return false;
            }
        }
        true
    }
}

/**
 * Your MyCalendarTwo object will be instantiated and called as such:
 * let obj = MyCalendarTwo::new();
 * let ret_1: bool = obj.book(start, end);
 */
// @lc code=end

use std::collections::HashMap;
struct MyCalendarTwo {
tree:HashMap<i32,Vec<i32>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendarTwo {

    fn new() -> Self {
        Self{tree:HashMap::new()}
    }
    
    fn book(&mut self, start: i32, end: i32) -> bool {
        fn update(start: i32, end: i32,val:i32,l:i32,r:i32,idx:i32,tree:&mut HashMap<i32,Vec<i32>>){
           if start>r|| end<l{
               return 
           }
           if start<=l && end>=r{
              tree.entry(idx).and_modify(|p| {p[0]+=val;p[1]+=val;}).or_insert(vec![val;2]);
               return 
           }
           let mid=(l+r)/2;
           update(start,end,val,l,mid,idx*2,tree);
           update(start,end,val,mid+1,r,idx*2+1,tree);
           let (p,lp,rp)=(tree.get(&idx).unwrap_or(&vec![0;2])[1],tree.get(&(idx*2)).unwrap_or(&vec![0;2])[0],tree.get(&(idx*2+1)).unwrap_or(&vec![0;2])[0]);
           tree.insert(idx,vec![p+lp.max(rp),p]);
       }
       update(start,end-1,1,0,1_000_000_000,1,&mut self.tree);
       if self.tree[&1][0]>2{
update(start,end-1,-1,0,1_000_000_000,1,&mut self.tree);
            false
       }else{
           true
       }
    }
}

/**
 * Your MyCalendarTwo object will be instantiated and called as such:
 * let obj = MyCalendarTwo::new();
 * let ret_1: bool = obj.book(start, end);
 */
/*
 * @lc app=leetcode id=729 lang=rust
 *
 * [729] My Calendar I
 */

// @lc code=start
use std::collections::BTreeMap;
struct MyCalendar {
 tree: BTreeMap<i32, i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendar {

    fn new() -> Self {
         Self{tree:BTreeMap::new()}
    }
    
    fn book(&mut self, start: i32, end: i32) -> bool {
        for (&s, _) in self.tree.range(start..).take(1) {
            if s < end { return false; }
        }
        for (_, &e) in self.tree.range(..start).rev().take(1) {
            if e > start { return false; }
        }
        self.tree.insert(start, end);
        true
    }
}

/**
 * Your MyCalendar object will be instantiated and called as such:
 * let obj = MyCalendar::new();
 * let ret_1: bool = obj.book(start, end);
 */
// @lc code=end

use std::collections::HashSet;
struct MyCalendar {
lazy:HashSet<i32>,
tree:HashSet<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendar {

    fn new() -> Self {
        Self{lazy:HashSet::new(),tree:HashSet::new()}
    }
    
    fn book(&mut self, start: i32, end: i32) -> bool {
        fn query(start: i32, end: i32,l: i32, r: i32,idx:i32,lazy:&HashSet<i32>,
tree:&HashSet<i32>)->bool{
            if start>r||end<l{
                return false
            }
            if lazy.contains(&idx){
                return true
            }
            if start<=l && end>=r{
                return tree.contains(&idx)
            }
            let mid=(l+r)/2;
            query(start,end,l,mid,idx*2,lazy,tree) || query(start,end,mid+1,r,idx*2+1,lazy,tree)
        }
        fn update(start: i32, end: i32,l: i32, r: i32,idx:i32,lazy:&mut HashSet<i32>,
tree:&mut HashSet<i32>){
            if start>r||end<l{
                return 
            }
            if start<=l && end>=r{
                 tree.insert(idx);
                 lazy.insert(idx);
                 return 
            }
            let mid=(l+r)/2;
            update(start,end,l,mid,idx*2,lazy,tree);
            update(start,end,mid+1,r,idx*2+1,lazy,tree);
            tree.insert(idx);
            if lazy.contains(&(idx*2)) &&  lazy.contains(&(idx*2+1)){
                lazy.insert(idx);
            }
        }
       if query(start,end-1,0,1_000_000_000,1,&self.lazy,&self.tree){
           return false
       }
       update(start,end-1,0,1_000_000_000,1,&mut self.lazy,&mut self.tree);
       true
    }
}

/**
 * Your MyCalendar object will be instantiated and called as such:
 * let obj = MyCalendar::new();
 * let ret_1: bool = obj.book(start, end);
 */
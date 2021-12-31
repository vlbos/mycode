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


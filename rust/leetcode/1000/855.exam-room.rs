/*
 * @lc app=leetcode id=855 lang=rust
 *
 * [855] Exam Room
 */

// @lc code=start
use std::collections::BTreeSet;
struct ExamRoom {
n:i32,
seats:BTreeSet<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ExamRoom {

    fn new(n: i32) -> Self {
        Self{n,seats:BTreeSet::new()}
    }
    
    fn seat(&mut self) -> i32 {
        let mut ans = 0;
        if !self.seats.is_empty(){
            let mut dis = 0;
            let mut prev = -1;
            for s in &self.seats{
                if prev!=-1{
                    let d = (*s-prev)/2;
                    if d >dis{
                        dis = d;
                        ans = prev+d;
                    }
                }else{
                dis = *s;
                }
                prev = *s;
            } 
            if self.n-1-prev>dis{
                ans = self.n-1;
            }
        }
        self.seats.insert(ans);
        ans
    }
    
    fn leave(&mut self, p: i32) {
        self.seats.remove(&p);
    }
}

/**
 * Your ExamRoom object will be instantiated and called as such:
 * let obj = ExamRoom::new(n);
 * let ret_1: i32 = obj.seat();
 * obj.leave(p);
 */
// @lc code=end


/*
 * @lc app=leetcode id=900 lang=rust
 *
 * [900] RLE Iterator
 */

// @lc code=start
struct RLEIterator {
encoding: Vec<i32>,
i:usize,
q:i32,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RLEIterator {

    fn new(encoding: Vec<i32>) -> Self {
        Self{encoding,i:0,q:0}
    }
    
    fn next(&mut self, n: i32) -> i32 {
        let mut n = n  ;
        while self.i<self.encoding.len(){
                if self.q+n>self.encoding[self.i]{
                     n-=self.encoding[self.i]-self.q;
                    self.q=0;
                    self.i+=2;
                    continue;
                }
                self.q+=n;
                return self.encoding[self.i+1];
        }
        -1
    }
}

/**
 * Your RLEIterator object will be instantiated and called as such:
 * let obj = RLEIterator::new(encoding);
 * let ret_1: i32 = obj.next(n);
 */
// @lc code=end


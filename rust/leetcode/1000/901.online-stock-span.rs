/*
 * @lc app=leetcode id=901 lang=rust
 *
 * [901] Online Stock Span
 */

// @lc code=start
struct StockSpanner {
s:Vec<(i32,i32)>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StockSpanner {

    fn new() -> Self {
        Self{s:Vec::new()}
    }
    
    fn next(&mut self, price: i32) -> i32 {
        let mut w = 1;
        while !self.s.is_empty() && self.s.last().unwrap().0<=price{
            w+=self.s.pop().unwrap().1;
        }
        self.s.push((price,w));
        w
    }
}

/**
 * Your StockSpanner object will be instantiated and called as such:
 * let obj = StockSpanner::new();
 * let ret_1: i32 = obj.next(price);
 */
// @lc code=end


/*
 * @lc app=leetcode id=1352 lang=rust
 *
 * [1352] Product of the Last K Numbers
 */

// @lc code=start
struct ProductOfNumbers {
stream:Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ProductOfNumbers {

    fn new() -> Self {
        Self{stream:Vec::new()}
    }
    
    fn add(&mut self, num: i32) {
        self.stream.push(num);
    }
    
    fn get_product(&self, k: i32) -> i32 {
        let k =k as usize;
        let i = if self.stream.len()>k {self.stream.len()-k}else{0};
        self.stream[i..].iter().product()
    }
}

/**
 * Your ProductOfNumbers object will be instantiated and called as such:
 * let obj = ProductOfNumbers::new();
 * obj.add(num);
 * let ret_2: i32 = obj.get_product(k);
 */
// @lc code=end


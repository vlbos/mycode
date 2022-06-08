/*
 * @lc app=leetcode id=703 lang=rust
 *
 * [703] Kth Largest Element in a Stream
 */

// @lc code=start
struct KthLargest {
k:usize,
        nums: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {

    fn new(k: i32, nums: Vec<i32>) -> Self {
        let k = k as usize;

        let mut nums= nums;
        if !nums.is_empty(){
        nums.sort();
        }
        Self{k,nums: if nums.len() > k {nums[nums.len()-k..].to_vec()}else {nums}}
    }
    
    fn add(&mut self, val: i32) -> i32 {
        self.nums.push(val);
        self.nums.sort();
        if (self.nums.len()>self.k){
             self.nums.remove(0);
        }       

        self.nums[0]
    }
}

/**
 * Your KthLargest object will be instantiated and called as such:
 * let obj = KthLargest::new(k, nums);
 * let ret_1: i32 = obj.add(val);
 */
// @lc code=end


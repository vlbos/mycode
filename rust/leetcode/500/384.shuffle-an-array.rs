/*
 * @lc app=leetcode id=384 lang=rust
 *
 * [384] Shuffle an Array
 */

// @lc code=start
struct Solution {
   nums: Vec<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    fn new(nums: Vec<i32>) -> Self {
        Self{nums}
    }
    
    fn reset(&self) -> Vec<i32> {
        self.nums.clone()
    }
    
    fn shuffle(&self) -> Vec<i32> {
        let mut tmp = self.nums.clone();
        let mut n = tmp.len();
        use rand::Rng;
        let mut rng = rand::thread_rng();
        for i in 0..n{
            let j = rng.gen_range(i,n);
            let t = tmp[i];
            tmp[i]=tmp[j];
            tmp[j]=t;
        }
        let tmp =tmp;
        tmp
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(nums);
 * let ret_1: Vec<i32> = obj.reset();
 * let ret_2: Vec<i32> = obj.shuffle();
 */
// @lc code=end


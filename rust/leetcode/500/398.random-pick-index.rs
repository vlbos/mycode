/*
 * @lc app=leetcode id=398 lang=rust
 *
 * [398] Random Pick Index
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
    
    fn pick(&self, target: i32) -> i32 {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let mut count = 0;
        let mut ans = 0;
        for (i,v) in self.nums.iter().enumerate(){
            if *v==target{
                count+=1;
                if rng.gen::<i32>()%count==0{
                    ans = i as i32;
                }
            }
        }
        ans
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(nums);
 * let ret_1: i32 = obj.pick(target);
 */
// @lc code=end


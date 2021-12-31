/*
 * @lc app=leetcode id=457 lang=rust
 *
 * [457] Circular Array Loop
 */

// @lc code=start
impl Solution {
    pub fn circular_array_loop(nums: Vec<i32>) -> bool {
        if nums.len() == 1 {
            return false;
        }
        let n = nums.len();

        let next = |i: usize,nums: &Vec<i32>| -> usize { 
        let n = n as i32;
(((i as i32+ nums[i] ) % n + n) % n ) as usize
};
        let mut nums = nums;
        for i in 0..n {
            if nums[i] == 0 {
                continue;
            }
            let mut slow = i;
            let mut fast = next(i,&nums);
            while nums[slow] * nums[fast] > 0 && nums[slow] * nums[next(fast,&nums)] > 0 {
                if slow == fast {
                    if slow != next(slow,&nums) {
                        return true;
                    } else {
                        break;
                    }
                }
                slow = next(slow,&nums);
                fast = next(next(fast,&nums),&nums);
            }
            let mut j = i;
            while nums[j] * nums[next(j,&nums)] > 0 {
                let t = j;
                j = next(j,&nums);
                nums[t] = 0;
            }
        }
        false
    }
}
// @lc code=end

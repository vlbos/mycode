/*
 * @lc app=leetcode id=448 lang=rust
 *
 * [448] Find All Numbers Disappeared in an Array
 */

// @lc code=start
impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut r = Vec::<i32>::new();
        let n = nums.len() as i32;
        for i in 0..nums.len() {
            let k = ((nums[i] - 1) % n) as usize;
            nums[k] += n;
        }
        for i in 0..n {
            if nums[i as usize] <= n {
                r.push((i + 1) as i32);
            }
        }
        r
    }
}
// @lc code=end


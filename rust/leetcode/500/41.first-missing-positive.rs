/*
 * @lc app=leetcode id=41 lang=rust
 *
 * [41] First Missing Positive
 */

// @lc code=start
impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let mut nums = nums;
        for i in 0..nums.len() {
            while nums[i] > 0 && nums[i] < n && nums[nums[i] as usize - 1] != nums[i] {
                let i1=nums[i] as usize - 1;
                nums.swap(i1, i);
            }
        }
        for (i, &v) in nums.iter().enumerate() {
            if v != i as i32 + 1 {
                return i as i32 + 1;
            }
        }
        n + 1
    }
}
// @lc code=end

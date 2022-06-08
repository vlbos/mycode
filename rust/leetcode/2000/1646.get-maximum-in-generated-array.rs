/*
 * @lc app=leetcode id=1646 lang=rust
 *
 * [1646] Get Maximum in Generated Array
 */

// @lc code=start
impl Solution {
    pub fn get_maximum_generated(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        let n = n as usize;
        let mut nums = vec![0; n + 1];
        nums[1] = 1;
        for i in 1..(n + 1) / 2 {
            let i2 = 2 * i;
            let i21 = 2 * i + 1;
            if i2 <= n {
                nums[i2] = nums[i];
            }
            if i21 <= n {
                nums[i21] = nums[i] + nums[i + 1];
            }
        }
        *(nums.iter().max().unwrap())
    }
}
// @lc code=end

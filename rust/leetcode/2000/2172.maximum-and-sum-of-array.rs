/*
 * @lc app=leetcode id=2172 lang=rust
 *
 * [2172] Maximum AND Sum of Array
 */

// @lc code=start
impl Solution {
    pub fn maximum_and_sum(nums: Vec<i32>, num_slots: i32) -> i32 {
        let n = nums.len();
        let mask_max = 3usize.pow(num_slots as u32);
        let mut f = vec![0; mask_max];
        for mask in 1..mask_max {
            let mut cnt = 0;
            let mut dummy = mask;
            for _ in 0..num_slots {
                cnt += dummy % 3;
                dummy /= 3;
            }
            if cnt > n {
                continue;
            }
            let mut dummy = mask;
            let mut w = 1;
            for i in 0..num_slots {
                if dummy % 3 > 0 {
                    f[mask] = f[mask].max(f[mask - w] + (nums[cnt - 1] & (i + 1)))
                }
                dummy /= 3;
                w *= 3;
            }
        }
        *f.iter().max().unwrap()
    }
}
// @lc code=end

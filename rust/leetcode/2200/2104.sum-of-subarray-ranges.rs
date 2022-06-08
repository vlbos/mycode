/*
 * @lc app=leetcode id=2104 lang=rust
 *
 * [2104] Sum of Subarray Ranges
 */

// @lc code=start
impl Solution {
    pub fn sub_array_ranges(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut lr = vec![vec![0; n]; 4];
        let mut ms = vec![Vec::new(); 2];

        for i in 0..n {
            for j in 0..2 {
                while !ms[j].is_empty()
                    && vec![
                        nums[ms[j][ms[j].len() - 1]] > nums[i],
                        nums[ms[j][ms[j].len() - 1]] <= nums[i],
                    ][j]
                {
                    ms[j].pop();
                }
                lr[j][i] = if ms[j].is_empty() {
                   -1
                } else {
                    ms[j][ms[j].len() - 1] as i32
                };
                ms[j].push(i);
            }
        }
        ms = vec![Vec::new(); 2];
        for i in (0..n).rev() {
            for j in 0..2 {
                while !ms[j].is_empty()
                    && vec![
                        nums[ms[j][ms[j].len() - 1]] >= nums[i],
                        nums[ms[j][ms[j].len() - 1]] < nums[i],
                    ][j]
                {
                    ms[j].pop();
                }
                lr[j+2][i]  = if ms[j].is_empty() {
                    n as i32
                } else {
                    ms[j][ms[j].len() - 1] as i32
                };
                ms[j].push(i);
            }
        }
        let (mut sum_max, mut sum_min) = (0, 0);
        for (i, &v) in nums.iter().enumerate() {
            let (ii, vv) = (i as i64, v as i64);
            let (min_left, max_left, min_right, max_right) = (
                lr[0][i] as i64,
                lr[1][i] as i64,
                lr[2][i] as i64,
                lr[3][i] as i64,
            );
            sum_max += (max_right - ii) * (ii - max_left) * vv;
            sum_min += (min_right - ii) * (ii - min_left) * vv;
        }
        sum_max - sum_min
    }
}
// @lc code=end

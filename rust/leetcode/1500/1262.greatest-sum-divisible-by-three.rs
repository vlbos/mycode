/*
 * @lc app=leetcode id=1262 lang=rust
 *
 * [1262] Greatest Sum Divisible by Three
 */

// @lc code=start
impl Solution {
    pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
         let mut r = vec![0;3];
        for &v in &nums{
            let mut rr = vec![0;3];
            for i in 0..3{
                rr[i]=r[i]+v;
            }
            for i in 0..3{
                let j = (rr[i] %3) as usize;
                r[j]=r[j].max(rr[i]);
            }
        }
        r[0]
    }
}
// @lc code=end


/*
 * @lc app=leetcode.cn id=628 lang=rust
 *
 * [628] 三个数的最大乘积
 */

// @lc code=start
impl Solution {
    pub fn maximum_product(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut x = 1;
        let mut n = nums.clone();
        n.sort();
        for i in 0..2{
           x*=n[i];
        }
        max = (x*n[2]).max(x*n[n.len()-1]);
        x=1;
        for i in 0..3{
            x*=n[n.len()-i-1];
        }
        x.max(max)
    }
}
// @lc code=end


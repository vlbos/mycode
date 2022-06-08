/*
 * @lc app=leetcode id=1822 lang=rust
 *
 * [1822] Sign of the Product of an Array
 */

// @lc code=start
impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        let mut n = 1;
        for i in &nums{
            n *= if *i < 0 {
                -1
            } else if *i > 0 {
                1
            } else {
                0
            };
            if n==0{
            return 0;
            }
        } 
        n
    }
}
// @lc code=end

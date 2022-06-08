/*
 * @lc app=leetcode id=1250 lang=rust
 *
 * [1250] Check If It Is a Good Array
 */

// @lc code=start
impl Solution {
    pub fn is_good_array(nums: Vec<i32>) -> bool {
        fn gcd(a: i32, b: i32)->i32 {
            if b == 0 {
                a
            } else {
                gcd(b, a % b)
            }
        }
        let mut g = nums[0];
        if g == 1 {
            return true;
        }
        for &num in nums.iter().skip(1) {
            g = gcd(g, num);
            if g == 1 {
                return true;
            }
        }
        false
    }
}
// @lc code=end

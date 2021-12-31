/*
 * @lc app=leetcode id=1979 lang=rust
 *
 * [1979] Find Greatest Common Divisor of Array
 */

// @lc code=start
impl Solution {
    pub fn find_gcd(nums: Vec<i32>) -> i32 {
        let gcd = |x: i32, y: i32| -> i32 {
            let (mut a, mut b) = if x > y { (x, y) } else { (y, x) };
            while b > 0 {
                let t = b;
                b = a % b;
                a = t;
            }
            a
        };
        let mut min = 1001;
        let mut max = 0;
        for n in &nums {
            if *n > max {
                max = *n;
            }
            if *n < min {
                min = *n;
            } 
        }
        gcd(min, max)
    }
}
// @lc code=end

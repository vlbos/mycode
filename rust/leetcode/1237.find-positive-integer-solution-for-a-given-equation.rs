/*
 * @lc app=leetcode id=1237 lang=rust
 *
 * [1237] Find Positive Integer Solution for a Given Equation
 */

// @lc code=start
/*
 * // This is the custom function interface.
 * // You should not implement it, or speculate about its implementation
 * struct CustomFunction;
 * impl CustomFunction {
 *    pub fn f(x:i32,y:i32)->i32{}
 * }
 */

impl Solution {
    pub fn find_solution(customfunction: &CustomFunction, z: i32) -> Vec<Vec<i32>> {
        let (mut x, mut y) = (1, 1000);
        let mut ans = Vec::new();
        while x <= 1000 && y >= 1 {
            let ret = customfunction.f(x, y);
            if ret < z {
                x += 1;
            } else if ret > z {
                y -= 1;
            } else {
                ans.push(vec![x, y]);
                x += 1;
                y -= 1;
            }
        }
        ans
    }
}
// @lc code=end

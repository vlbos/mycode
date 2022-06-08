/*
 * @lc app=leetcode id=1835 lang=rust
 *
 * [1835] Find XOR Sum of All Pairs Bitwise AND
 */

// @lc code=start
impl Solution {
    pub fn get_xor_sum(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
       arr1.into_iter().reduce(|a,x| a^x).unwrap()&arr2.into_iter().reduce(|a,x| a^x).unwrap()
    }
}
// @lc code=end


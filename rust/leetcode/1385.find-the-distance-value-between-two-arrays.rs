/*
 * @lc app=leetcode id=1385 lang=rust
 *
 * [1385] Find the Distance Value Between Two Arrays
 */

// @lc code=start
impl Solution {
    pub fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
        let mut ans = arr1.len();
        for a in &arr1{
            for b in &arr2{
                if (*a-*b).abs()<=d{
                    ans-=1;
                    break;
                }
            }
        }
        ans as i32
    }
}
// @lc code=end


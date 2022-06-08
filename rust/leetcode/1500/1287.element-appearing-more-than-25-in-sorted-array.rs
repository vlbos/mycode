/*
 * @lc app=leetcode id=1287 lang=rust
 *
 * [1287] Element Appearing More Than 25% In Sorted Array
 */

// @lc code=start
impl Solution {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        let mut c = 0;
        let mut n = arr[0];
        for a in &arr {
            if *a == n {
                c += 1;
            } else {
                n = *a;
                c = 1;
            }
            if c * 4 > arr.len() {
                return *a;
            }
        }
        n
    }
}
// @lc code=end

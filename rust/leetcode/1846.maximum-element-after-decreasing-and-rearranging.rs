/*
 * @lc app=leetcode id=1846 lang=rust
 *
 * [1846] Maximum Element After Decreasing and Rearranging
 */

// @lc code=start
impl Solution {
    pub fn maximum_element_after_decrementing_and_rearranging(arr: Vec<i32>) -> i32 {
        let mut arr = arr;
        arr.sort();
        arr[0]=1;
        for i in 1..arr.len() {
            arr[i] = arr[i].min(arr[i - 1] + 1);
        }
        arr[arr.len() - 1]
    }
}
// @lc code=end

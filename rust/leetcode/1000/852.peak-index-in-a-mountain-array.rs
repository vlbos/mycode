/*
 * @lc app=leetcode id=852 lang=rust
 *
 * [852] Peak Index in a Mountain Array
 */

// @lc code=start
impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        let mut mid = arr.len() / 2;
        let mut i = 0;
        let mut j = arr.len();
        while i <= j {
            mid = (i + j) / 2;
            if mid > 0 && mid < arr.len() - 1 {
                if arr[mid] > arr[mid - 1] && arr[mid] > arr[mid + 1] {
                    return mid as i32;
                } else if arr[mid] > arr[mid - 1] && arr[mid] < arr[mid + 1] {
                    i = mid + 1;
                } else if mid > 0 {
                    j = mid;
                }
            } else {
                break;
            }
        }
        0
    }
}
// @lc code=end


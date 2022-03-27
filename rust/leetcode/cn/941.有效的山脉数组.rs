/*
 * @lc app=leetcode.cn id=941 lang=rust
 *
 * [941] 有效的山脉数组
 */

// @lc code=start
impl Solution {
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        if arr.len() < 3 {
            return false;
        }
        for i in 0..arr.len() - 1 {
            if arr[i] == arr[i + 1] {
                return false;
            }
        }
        let mut i = 0;
        let mut j = arr.len() - 1;
        let mut mid = arr.len();
        while i < j {
            mid = (i + j) / 2;
            if mid == 0 || mid == arr.len() - 1 {
                mid = arr.len();
                break;
            }
            if mid > 0 && mid < arr.len() - 1 && arr[mid] > arr[mid - 1] && arr[mid] > arr[mid + 1]
            {
                break;
            }

            if arr[mid] > arr[mid - 1] && arr[mid] < arr[mid + 1] {
                i = mid + 1;
            } else if arr[mid] < arr[mid - 1] && arr[mid] > arr[mid + 1] {
                j = mid;
            } else {
                mid = arr.len();
                break;
            }
        }
        if mid != arr.len() {
            for i in 0..mid {
                if arr[i] > arr[i + 1] {
                    return false;
                }
            }
            for i in mid..arr.len() - 1 {
                if arr[i] < arr[i + 1] {
                    return false;
                }
            }
            return true;
        }
        false
    }
}
// @lc code=end

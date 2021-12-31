/*
 * @lc app=leetcode id=658 lang=rust
 *
 * [658] Find K Closest Elements
 */

// @lc code=start
impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let k = k as usize;
        if x <= *arr.first().unwrap() {
            return arr[0..k].to_vec();
        } else if x >= *arr.last().unwrap() {
            return arr[arr.len() - k..].to_vec();
        }
        let mut l = 0;
        let mut r = arr.len() - 1;
        let mut pre = arr.len();
        while l <= r {
            let mid = (l + r) / 2;
            if arr[mid] > x {
                r = mid;
            } else {
                l = mid + 1;
            }
            if pre == mid {
                break;
            }
            pre = mid;
        }
        let mut low = if l >= k + 1 { l - k - 1 } else { 0 };
        let mut high = if l + k - 1 < arr.len() {
            l + k - 1
        } else {
            arr.len() - 1
        };
        while high - low > k - 1 {
            let lv = (arr[low] - x).abs();
            let hv = (arr[high] - x).abs();
            if hv < lv {
                low += 1;
            } else {
                high -= 1;
            }
        }
        arr[low..high + 1].to_vec()
    }
}
// @lc code=end

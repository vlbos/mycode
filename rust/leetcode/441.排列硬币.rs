/*
 * @lc app=leetcode.cn id=441 lang=rust
 *
 * [441] 排列硬币
 */

// @lc code=start
impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        let n = n as i64;
        let mut start = 1 as i64;
        let mut end = n as i64;
        let mut mid = (start + end) / 2;
        while start <= end {
            let m = ((mid + 1) * mid) / 2;
            if m <= n {
                if m + mid + 1 > n {
                    return mid as i32;
                }
                start = mid + 1;
            } else {
                end = mid - 1;
            }
            mid = (start + end) / 2;
        }
        0
    }
}
// @lc code=end

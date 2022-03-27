/*
 * @lc app=leetcode id=1109 lang=rust
 *
 * [1109] Corporate Flight Bookings
 */

// @lc code=start
impl Solution {
    pub fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
        let mut ans =vec![0;n as usize];
        for b in &bookings{
            ans[b[0] as usize-1]+=b[2];
            if b[1]<n{
                ans[b[1] as usize]-=b[2];
            }
        }
        for i in 1..n as usize{
            ans[i]+=ans[i-1];
        }
        ans
    }
}
// @lc code=end


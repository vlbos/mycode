/*
 * @lc app=leetcode id=1184 lang=rust
 *
 * [1184] Distance Between Bus Stops
 */

// @lc code=start
impl Solution {
    pub fn distance_between_bus_stops(distance: Vec<i32>, start: i32, destination: i32) -> i32 {
        let mut cw = 0;
        let mut ccw=0;
        let n = distance.len() as i32;
        let s = start.min(destination);
        let d = start.max(destination);
        for i in s..d{
            cw+=distance[i as usize];
        }
        let mut i = d;
        while i%n!=s{
            ccw+=distance[i as usize];
            i=(i+1)%n;
        }
        cw.min(ccw)
    }
}
// @lc code=end


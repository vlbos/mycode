/*
 * @lc app=leetcode id=539 lang=rust
 *
 * [539] Minimum Time Difference
 */

// @lc code=start
impl Solution {
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
     let mut t = time_points.iter()
            .map(|x| {
           x.split(':')
                    .map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>()
            }).map(|x| x[0]*60+x[1])
            .collect::<Vec<i32>>();
        t.sort();
        let m = t.windows(2).map(|a|a[1]-a[0]).min().unwrap();
        m.min(1440+t.first().unwrap()-t.last().unwrap())
    }
}
// @lc code=end

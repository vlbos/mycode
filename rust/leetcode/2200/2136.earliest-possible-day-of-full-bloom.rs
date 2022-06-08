/*
 * @lc app=leetcode id=2136 lang=rust
 *
 * [2136] Earliest Possible Day of Full Bloom
 */

// @lc code=start
impl Solution {
    pub fn earliest_full_bloom(plant_time: Vec<i32>, grow_time: Vec<i32>) -> i32 {
        let n = plant_time.len();
        let mut id: Vec<usize> = (0..n).collect();
        id.sort_by(|&a, &b| grow_time[b].cmp(&grow_time[a]));
        let (mut prev, mut ans) = (0, 0);
        for &i in &id {
            ans = ans.max(prev + plant_time[i] + grow_time[i]);
            prev +=plant_time[i];
        }
        ans
    }
}
// @lc code=end

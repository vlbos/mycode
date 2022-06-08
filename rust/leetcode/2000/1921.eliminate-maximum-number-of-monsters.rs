/*
 * @lc app=leetcode id=1921 lang=rust
 *
 * [1921] Eliminate Maximum Number of Monsters
 */

// @lc code=start
impl Solution {
    pub fn eliminate_maximum(dist: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut time = Vec::new();
        for (i, d) in dist.iter().enumerate() {
            time.push((d-1) / speed[i]);
        }
        time.sort();
        for (i, &t) in time.iter().enumerate() {
            let ii = i as i32;
            if t < ii {
                return ii;
            }
        }
        time.len() as _
    }
}
// @lc code=end

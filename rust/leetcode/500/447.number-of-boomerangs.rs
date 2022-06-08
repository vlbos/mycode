/*
 * @lc app=leetcode id=447 lang=rust
 *
 * [447] Number of Boomerangs
 */

// @lc code=start
impl Solution {
    pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;

        for p in &points{
            let mut m = std::collections::HashMap::new();
            for q in &points{
                 let d  = (p[0]-q[0]).pow(2)+(p[1]-q[1]).pow(2);
                 *(m.entry(d).or_insert(0))+=1;
            }
            for (_,v) in &m{
                    ans += v*(v-1);
            }
        }
      
        ans
    }
}
// @lc code=end


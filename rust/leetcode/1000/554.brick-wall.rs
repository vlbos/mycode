/*
 * @lc app=leetcode id=554 lang=rust
 *
 * [554] Brick Wall
 */

// @lc code=start
impl Solution {
    pub fn least_bricks(wall: Vec<Vec<i32>>) -> i32 {
        let mut m = std::collections::HashMap::new();
        for w in &wall{
            let mut sum =0;
            for i in 0..w.len()-1{
                 sum+=w[i];
                 *m.entry(sum).or_insert(0)+=1;
            }
        }
        let ans = match m.iter().map(|(k,v)|v).max(){ Some(v)=>*v,_=>0};
        wall.len() as i32-ans
    }
}
// @lc code=end


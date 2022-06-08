/*
 * @lc app=leetcode id=735 lang=rust
 *
 * [735] Asteroid Collision
 */

// @lc code=start
impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut s = Vec::new();
        for a in &asteroids{
            let cur = *a;
            let mut flag = true;
            while !s.is_empty() && cur<0 && *s.last().unwrap()>0 {
                if *s.last().unwrap()<  -cur{
                    s.pop();
                    continue;
                }
                if  *s.last().unwrap()==-cur{
                    s.pop();
                }
                flag =false;
                break;
            }
            if flag{
            s.push(cur);

            }
        }
        s
    }
}
// @lc code=end


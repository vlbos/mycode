/*
 * @lc app=leetcode id=2126 lang=rust
 *
 * [2126] Destroying Asteroids
 */

// @lc code=start
impl Solution {
    pub fn asteroids_destroyed(mass: i32, asteroids: Vec<i32>) -> bool {
        let mut asteroids = asteroids;
        asteroids.sort();
        let mut m = mass as i64;
        for &a in &asteroids {
            let aa =  a as i64;
            if m < aa{
                return false;
            }
            m += aa;
        }
        true
    }
}
// @lc code=end

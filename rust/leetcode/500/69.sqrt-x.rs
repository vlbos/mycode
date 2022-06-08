/*
 * @lc app=leetcode id=69 lang=rust
 *
 * [69] Sqrt(x)
 */

// @lc code=start
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
         if (x == 0) {
            return 0;
        }
        let x = x as f64;
        let mut z: f64 = x / 2.0;
        let mut pz = 0.0;
        loop {
            pz = z;
            z = (z + x / z) / 2.0;
            if pz - z < 1.0 {
                break;
            }
        }
        z as i32
    }
}
// @lc code=end


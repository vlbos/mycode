/*
 * @lc app=leetcode id=1710 lang=rust
 *
 * [1710] Maximum Units on a Truck
 */

// @lc code=start
impl Solution {
    pub fn maximum_units(box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
        let mut tb = vec![0; 1001];
        for n in &box_types {
            tb[n[1] as usize] += n[0];
        }
        let mut sum = 0;
        let mut cnt = 0;
        for i in (0..tb.len()).rev() {
            let n = tb[i];
            if n > 0 {
                if cnt + n <= truck_size {
                    sum += n * i as i32;
                    cnt += n;
                } else if cnt < truck_size {
                    sum += (truck_size-cnt) * i as i32;
                    cnt += truck_size-cnt;
                    break;
                }
            }
        }
        sum
    }
}
// @lc code=end

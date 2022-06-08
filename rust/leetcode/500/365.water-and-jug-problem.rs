/*
 * @lc app=leetcode id=365 lang=rust
 *
 * [365] Water and Jug Problem
 */

// @lc code=start
impl Solution {
    pub fn can_measure_water(jug1_capacity: i32, jug2_capacity: i32, target_capacity: i32) -> bool {
        let gcd = |a: i32, b: i32| -> i32 {
            let mut m = a;
            let mut n = b;
            while m !=n {
                if m > n {
                    let t = m;
                    m = n;
                    n = t;
                }
                let t = m;
                m = n - m;
                n = t;
            }
            m
        };
        if jug1_capacity + jug2_capacity < target_capacity {
            return false;
        } else if jug1_capacity == 0 || jug2_capacity == 0 {
            return target_capacity == 0 || jug1_capacity + jug2_capacity == target_capacity;
        }
        target_capacity % gcd(jug1_capacity, jug2_capacity) == 0
    }
}
// @lc code=end

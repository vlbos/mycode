/*
 * @lc app=leetcode id=853 lang=rust
 *
 * [853] Car Fleet
 */

// @lc code=start
impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        if position.len() == 0 {
            return 0;
        }

        let mut cars = position.iter().zip(speed.iter()).collect::<Vec<_>>();
        cars.sort();
        let (&p, &s) = cars.pop().unwrap();
        let mut time = (target - p) as f64 / s as f64;
        let mut ret = 1;

        while let Some((&p, &s)) = cars.pop() {
            let t = (target - p) as f64 / s as f64;
            if time < t {
                ret += 1;
                time = t;
            }
        }

        ret
    }
}
// @lc code=end


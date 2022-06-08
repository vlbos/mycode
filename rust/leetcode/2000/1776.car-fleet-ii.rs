/*
 * @lc app=leetcode id=1776 lang=rust
 *
 * [1776] Car Fleet II
 */

// @lc code=start
impl Solution {
    pub fn get_collision_times(cars: Vec<Vec<i32>>) -> Vec<f64> {
        let mut ans = vec![0.0; cars.len()];
        let mut s: Vec<usize> = Vec::new();
        for (i, car) in cars.iter().enumerate().rev() {
            while !s.is_empty() {
                let last = s[s.len() - 1];
                if cars[last][1] >= car[1] {
                    s.pop();
                } else {
                    if ans[last] < 0.0 {
                        break;
                    }
                    let d = ans[last] * (car[1] - cars[last][1]) as f64;
                    if d > (cars[last][0] - car[0]) as f64 {
                        break;
                    } else {
                        s.pop();
                    }
                }
            }

            ans[i] = if s.is_empty() {
                -1.0
            } else {
                let last = s[s.len() - 1];
                (cars[last][0] - car[0]) as f64 / (car[1] - cars[last][1]) as f64
            };
            s.push(i);
        }
        ans
    }
}
// @lc code=end

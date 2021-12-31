/*
 * @lc app=leetcode id=1588 lang=rust
 *
 * [1588] Sum of All Odd Length Subarrays
 */

// @lc code=start
impl Solution {
    pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
        if arr.len() == 1 {
            return arr[0];
        }
        let mut sum = arr.iter().sum::<i32>();
        if arr.len() <= 3 {
            return sum + sum * (arr.len() - 2) as i32;
        }
        if arr.len() % 2 != 0 {
            sum *= 2;
        }
        let mut ps = vec![0; arr.len()];
        let mut presum = 0;
        for i in 0..arr.len() {
            presum += arr[i as usize];
            ps[i] = presum;
        }
        for j in (3..arr.len()).step_by(2) {
            for i in j - 1..ps.len() {
                sum += ps[i];
                if i + 1 - j > 0 {
                    sum -= ps[i - j];
                }
            }
        }

        sum
    }
}
// @lc code=end

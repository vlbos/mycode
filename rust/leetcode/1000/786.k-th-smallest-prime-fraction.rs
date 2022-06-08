/*
 * @lc app=leetcode id=786 lang=rust
 *
 * [786] K-th Smallest Prime Fraction
 */

// @lc code=start
impl Solution {
    pub fn kth_smallest_prime_fraction(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let n = arr.len();
        let (mut left, mut right) = (0.0, 1.0);
        loop {
            let mid = (left + right) / 2.0;
            let mut i = -1;
            let mut count = 0;
            let (mut x, mut y) = (0, 1);
            for j in 1..n {
                while (arr[(i + 1) as usize] as f64) / (arr[j] as f64) < mid {
                    i += 1;
                    if arr[i as usize] * y > arr[j] * x {
                        x = arr[i as usize];
                        y = arr[j];
                    }
                }
                count += i + 1;
            }
            if count == k {
                return vec![x, y];
            }
            if count < k {
                left = mid;
            } else {
                right = mid;
            }
        }
        Vec::new()
    }
}
// @lc code=end

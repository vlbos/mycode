/*
 * @lc app=leetcode id=16 lang=rust
 *
 * [16] 3Sum Closest
 */

// @lc code=start
impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut n = nums;
        n.sort();
        let mut ans = i32::MAX- target;
        for i in 0..n.len() {
            let mut l = i + 1;
            let mut r = n.len() - 1;
            while l < r {
                let sum = n[i] + n[l] + n[r];
                if sum == target {
                    return sum;
                } else if sum > target {
                    if (sum - target).abs() < (ans - target).abs() { 
                        ans = sum;
                    }
                    r -= 1;
                } else {
                    if (sum - target).abs() < (ans - target).abs() {
                        ans = sum;
                    }
                    l += 1;
                }
            }
        }
        ans
    }
}
// @lc code=end

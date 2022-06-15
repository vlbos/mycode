/*
 * @lc app=leetcode id=2164 lang=rust
 *
 * [2164] Sort Even and Odd Indices Independently
 */

// @lc code=start
impl Solution {
    pub fn sort_even_odd(nums: Vec<i32>) -> Vec<i32> {
        let mut nums: Vec<(i32,usize)> = nums.into_iter().enumerate().map(|(i, v)| (v, i)).collect();
        nums.sort();
        let n = nums.len();
        let mut i = 0;
        let mut j = n - if n % 2 > 0 { 2 } else { 1 };
        let mut ans = vec![0; n];
        for &(num, k) in &nums {
            if k % 2 ==0  {
                ans[i] = num;
                i += 2;
            } else {
                ans[j] = num;
                j -= 2;
            }
        }
        ans
    }
}
// @lc code=end

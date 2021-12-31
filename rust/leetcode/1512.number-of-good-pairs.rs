/*
 * @lc app=leetcode id=1512 lang=rust
 *
 * [1512] Number of Good Pairs
 */

// @lc code=start
impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut a = vec![0;101];
        let mut ans = 0;

        for n in &nums{
            let i = *n as usize;
            ans+=a[i];
            a[i]+=1;
        }
        ans
    }
}
// @lc code=end


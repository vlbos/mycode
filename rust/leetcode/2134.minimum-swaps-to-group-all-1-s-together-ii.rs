/*
 * @lc app=leetcode id=2134 lang=rust
 *
 * [2134] Minimum Swaps to Group All 1's Together II
 */

// @lc code=start
impl Solution {
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let k = nums.iter().filter(|&x| *x == 1).count();
        if k==0{
            return 0;
        }
        let n = nums.len();
        let mut cur = nums[..k].iter().filter(|&x| *x == 0).count() as i32;
        let mut ans = cur ;
        for i in 1..n {
            if nums[i - 1] == 0 {
                cur -= 1;
            }
            if nums[(i + k - 1) % n] == 0 {
                cur += 1;
            }
            ans = ans.min(cur);
        }
        ans
    }
}
// @lc code=end

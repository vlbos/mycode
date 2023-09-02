/*
 * @lc app=leetcode id=42 lang=rust
 *
 * [42] Trapping Rain Water
 */

// @lc code=start
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut ans = 0;
        let (mut l, mut r) = (0, height.len() - 1);
        let (mut lm, mut rm) = (0, 0);
        while l < r {
            lm = lm.max(height[l]);
            rm = rm.max(height[r]);
            if height[l] < height[r] {
                ans += lm - height[l];
                l += 1;
            } else {
                ans += rm - height[r];
                r -= 1;
            }
        }
        ans
    }
}
// @lc code=end
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let n=height.len();
        let mut left_max=vec![0;n];
        left_max[0]=height[0];
        for i in 1..n{
            left_max[i]=left_max[i-1].max(height[i]);
        }
        let mut right_max=vec![0;n];
        right_max[n-1]=height[n-1];
        for i in (0..n-1).rev(){
            right_max[i]=right_max[i+1].max(height[i]);
        }
        left_max.into_iter().zip(right_max).zip(height).map(|((l,r),h)| l.min(r)-h).sum::<i32>()
    }
}
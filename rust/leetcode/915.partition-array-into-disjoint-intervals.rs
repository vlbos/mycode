/*
 * @lc app=leetcode id=915 lang=rust
 *
 * [915] Partition Array into Disjoint Intervals
 */

// @lc code=start
impl Solution {
    pub fn partition_disjoint(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut lmax = vec![0;n];
        let mut rmin = vec![0;n];
        let mut max = nums[0];
        for (i,v) in nums.iter().enumerate(){
            max = max.max(*v);
            lmax[i]=max;
        }
        let mut min = *nums.last().unwrap();
        for i in (0..n).rev(){
            min = min.min(nums[i]);
            rmin[i]=min;
        }
        let mut ans = 0;
        for i in 1..n{
            if lmax[i-1]<=rmin[i]{
                ans=i as i32;
                break;
            }
        }
        ans
    }
}
// @lc code=end


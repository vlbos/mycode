/*
 * @lc app=leetcode id=300 lang=rust
 *
 * [300] Longest Increasing Subsequence
 */

// @lc code=start
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return nums.len() as i32;
        }
        let mut dp = vec![1; nums.len()];
        for i in 1..nums.len() {
            for j in 0..i {
                if nums[j] < nums[i] {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
        }
        *(dp.iter().max().unwrap())
    }
}
// @lc code=end
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let n=nums.len();
        let mut d=vec![0;n+1];
        let mut len=1;
        d[len]=nums[0];
        for (i,&num) in nums.iter().enumerate(){
            if d[len]<num{
                len+=1;
                d[len]=num;
                continue
            }
            let (mut l,mut r)=(1,len);
            let mut pos=0;
            while l<=r{
                let mid=(l+r)/2;
                if d[mid]<num{
                    pos=mid;
                    l=mid+1;
                }else{
                    if mid==0{
                        break
                    }
                    r=mid-1;
                }
            }
            d[pos+1]=num;
        }
        len as _
    }
}
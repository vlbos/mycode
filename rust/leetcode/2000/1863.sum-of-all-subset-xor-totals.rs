/*
 * @lc app=leetcode id=1863 lang=rust
 *
 * [1863] Sum of All Subset XOR Totals
 */

// @lc code=start
impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        for n in &nums {
            res |= *n;
        }
        res << nums.len() - 1
    }
}
// @lc code=end
impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        fn dfs(idx:usize,val:i32,nums: &Vec<i32>,ans:&mut i32){
            if idx==nums.len(){
                *ans+=val;
                return
            }
            dfs(idx+1,val^nums[idx],nums,ans);
            dfs(idx+1,val,nums,ans);
        }
        let mut ans=0;
        dfs(0,0,&nums,&mut ans);
        ans
    }
}
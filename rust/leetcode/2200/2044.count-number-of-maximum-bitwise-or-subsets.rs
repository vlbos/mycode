/*
 * @lc app=leetcode id=2044 lang=rust
 *
 * [2044] Count Number of Maximum Bitwise-OR Subsets
 */

// @lc code=start
impl Solution {
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut max_value = 0;
        let mut cnt = 0;
        let n1 = 1 << n;
        for i in 1..n1 {
            let or_val = nums
                .iter()
                .enumerate()
                .filter(|(j, _)| ((i >> j) & 1) > 0)
                .map(|x|x.1).cloned().reduce(|acc, v| acc|v).unwrap();
            if or_val > max_value {
                max_value = or_val;
                cnt = 1;
            } else if or_val == max_value {
                cnt += 1;
            }
        }
        cnt
    }
}
// @lc code=end
impl Solution {
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        fn dfs(pos:usize,val:i32,nums: &Vec<i32>,ans:&mut Vec<i32>){
            if pos==nums.len(){
                if val>ans[1]{
                    *ans=vec![1,val];
                }else if val==ans[1]{
                    ans[0]+=1;
                }
                return
            }
            dfs(pos+1,val|nums[pos],nums,ans);
            dfs(pos+1,val,nums,ans);
        }
        let mut ans=vec![0;2];
        dfs(0,0,&nums,&mut ans);
        ans[0]
    }
}
/*
 * @lc app=leetcode id=740 lang=rust
 *
 * [740] Delete and Earn
 */

// @lc code=start
impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        let max = *nums.iter().max().unwrap() as usize;
        let mut sum = vec![0;max+1];
        for n in &nums{
            sum[(*n) as usize]+=*n;
        }
        if sum.len()==1{
            return sum[0];
        } 
        let mut first = sum[0];
        let mut second=sum[0].max(sum[1]);
        for i in 2..sum.len(){
                let t = second;
                second = second.max(first+sum[i]);
                first = t;
        }
        second
    }
}
// @lc code=end


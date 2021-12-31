/*
 * @lc app=leetcode id=1365 lang=rust
 *
 * [1365] How Many Numbers Are Smaller Than the Current Number
 */

// @lc code=start
impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut min = 101;
        let mut max = -1;
        for n in &nums {
            if *n > max {
                max = *n;
            }
            if *n < min {
                min = *n;
            }
        }
        if min==max{
            return vec![0;nums.len()];
        }
        let s = (max-min+1) as usize;
        let mut r = vec![0;s];
        for n in &nums {
            r[(*n-min) as usize]+=1;
        }
        let mut ps = vec![0;s];
        for i in 1..ps.len(){
            ps[i]=ps[i-1]+r[i-1];
        }
        let mut ans = Vec::new();
        for n in &nums {
            ans.push(ps[(*n-min) as usize]);
        }
        ans
    }
}
// @lc code=end

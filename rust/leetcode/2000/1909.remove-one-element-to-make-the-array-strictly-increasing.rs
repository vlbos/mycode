/*
 * @lc app=leetcode id=1909 lang=rust
 *
 * [1909] Remove One Element to Make the Array Strictly Increasing
 */

// @lc code=start
impl Solution {
    pub fn can_be_increasing(nums: Vec<i32>) -> bool {
        let mut first = 0;
        let mut second = 0;
        let mut cnt = 0;
        for n in &nums{
            if first==0{
                first = *n;
            }else if *n <= first && *n <= second{
                cnt+=1;
            }else if *n>first{
                second=first;
                first =*n;
            }else if *n>second{
                first =*n;
                cnt+=1;
            }
        }
        cnt < 2
    }
}
// @lc code=end

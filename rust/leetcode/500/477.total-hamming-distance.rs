/*
 * @lc app=leetcode id=477 lang=rust
 *
 * [477] Total Hamming Distance
 */

// @lc code=start
impl Solution {
    pub fn total_hamming_distance(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let mut ans = 0;
        for i in 0..30{
            let mut c= 0;
            for v in &nums{
                if ((*v)>>i)&1!=0{
                    c+=1;
                }
            }
            ans +=   c*(n-c);
        }
        ans
    }
}
// @lc code=end


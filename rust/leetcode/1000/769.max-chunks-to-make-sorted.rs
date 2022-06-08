/*
 * @lc app=leetcode id=769 lang=rust
 *
 * [769] Max Chunks To Make Sorted
 */

// @lc code=start
impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let mut max =0;
        let mut ans = 0;
        for (i,v) in arr.iter().enumerate(){
            max = max.max(*v);
            if i as i32 ==max{
            ans+=1;
            }
        }
        ans
    }
}
// @lc code=end


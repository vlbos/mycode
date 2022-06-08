/*
 * @lc app=leetcode id=560 lang=rust
 *
 * [560] Subarray Sum Equals K
 */

// @lc code=start
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = 0;
        let mut pre = 0;
        let mut m = std::collections::HashMap::new();
        m.insert(0,1);
        for n in &nums{
            pre+=*n;
            if let Some(cnt)=m.get(&(pre-k)){
            ans+=cnt;
            }
            *m.entry(pre).or_insert(0)+=1;
        }
        ans
    }
}
// @lc code=end


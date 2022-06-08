/*
 * @lc app=leetcode id=2025 lang=rust
 *
 * [2025] Maximum Number of Ways to Partition an Array
 */

// @lc code=start
impl Solution {
    pub fn ways_to_partition(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        use std::collections::HashMap;
        let mut pre = Vec::new();
        let mut sum = 0;
        let mut cntr = HashMap::new();
        for &num in &nums {
            sum += num;
            if !pre.is_empty() {
                *cntr.entry(pre[pre.len() - 1]).or_insert(0) += 1;
            }
            pre.push(sum);
        }
        let total = pre[n - 1];
        let mut ans = 0;
        if total % 2 == 0 {
            ans = *cntr.get(&(total / 2)).unwrap_or(&0);
        }
        println!("{}",ans);
        let mut cntl = HashMap::new();
        for (i, &s) in pre.iter().enumerate() {
            let d = k - nums[i];
            if (total + d) % 2 == 0 {
                ans = ans.max(
                    *cntl.get(&((total + d) / 2)).unwrap_or(&0)
                        + *cntr.get(&((total - d) / 2)).unwrap_or(&0),
                );
            }
            *cntl.entry(s).or_insert(0) += 1;
            *cntr.entry(s).or_insert(0) -= 1;
        }
        ans
    }
}
// @lc code=end

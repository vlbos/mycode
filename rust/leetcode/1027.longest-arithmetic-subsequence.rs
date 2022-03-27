/*
 * @lc app=leetcode id=1027 lang=rust
 *
 * [1027] Longest Arithmetic Subsequence
 */

// @lc code=start
impl Solution {
    pub fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        use std::collections::HashSet;
        let mut ans = 2;
        let mut hs = HashSet::new();
        for i in 0..nums.len(){
            for j in i+1..nums.len(){
                hs.insert(nums[j]-nums[i]);
            }
        }
        let longest_seq = |d: i32| -> i32 {
            let mut ans = 1;
            let mut m = HashMap::new();
            for &v in &nums {
                let cnt = *m.get(&(v - d)).unwrap_or(&0);
                m.insert(v, cnt + 1);
                ans = ans.max(cnt + 1);
            }
            ans
        };
        for &d in &hs {
            ans = ans.max(longest_seq(d));
        }
        ans
    }
}
// @lc code=end

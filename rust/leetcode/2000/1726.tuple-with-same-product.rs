/*
 * @lc app=leetcode id=1726 lang=rust
 *
 * [1726] Tuple with Same Product
 */

// @lc code=start
impl Solution {
    pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut m = HashMap::new();
        let n = nums.len();
        for  i in  0..n{
            for j in i+1..n {
                *m.entry(nums[i] * nums[j]).or_insert(0)+=1;
            }
        }
        let mut ans = 0;
        for (_, &n) in &m {
            if n <2 {
                continue;
            }
            ans +=( n*(n-1)*4) as i32;
        }
        ans
    }
}
// @lc code=end

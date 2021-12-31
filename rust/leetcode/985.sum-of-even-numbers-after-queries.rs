/*
 * @lc app=leetcode id=985 lang=rust
 *
 * [985] Sum of Even Numbers After Queries
 */

// @lc code=start
impl Solution {
    pub fn sum_even_after_queries(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
         let mut r = Vec::new();
        let mut s = 0;
        for n in &nums {
            if *n % 2 == 0 {
                s += *n;
            }
        }
        let mut nn = nums;
        for q in &queries {
            let index = q[1] as usize;
            let val = q[0];
            if nn[index] % 2 == 0 {
                s -= nn[index];
            }
            nn[index] += val;
            if nn[index] % 2 == 0 {
                s += nn[index];
            }
            r.push(s);
        }
        r
    }
}
// @lc code=end


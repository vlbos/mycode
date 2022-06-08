/*
 * @lc app=leetcode id=982 lang=rust
 *
 * [982] Triples with Bitwise AND Equal To Zero
 */

// @lc code=start
impl Solution {
    pub fn count_triplets(nums: Vec<i32>) -> i32 {
        let mut num2cnt = std::collections::HashMap::new();
        for &vi in &nums {
            for &vj in &nums {
                *num2cnt.entry(vi & vj).or_insert(0) += 1;
            }
        }
        let mut ans = 0;
        for (&num, &cnt) in &num2cnt {
            for &vi in &nums {
                if num & vi == 0 {
                    ans += cnt;
                }
            }
        }
        ans
    }
}
// @lc code=end

/*
 * @lc app=leetcode id=1589 lang=rust
 *
 * [1589] Maximum Sum Obtained of Any Permutation
 */

// @lc code=start
impl Solution {
    pub fn max_sum_range_query(nums: Vec<i32>, requests: Vec<Vec<i32>>) -> i32 {
        let n = nums.len();
        let mut counts = vec![0; n];
        for r in &requests {
            counts[r[0] as usize] += 1;
            let r1 = r[1] as usize + 1;
            if r1 < n {
                counts[r1] -= 1;
            }
        }
        for i in 1..n {
            counts[i] += counts[i - 1];
        }
        counts.sort();
        let mut nums = nums;
        nums.sort();
        let ans: i64 = nums
            .iter()
            .zip(counts)
            .map(|(&n, c)| (((n as i64) * (c as i64)) % 1000_000_007))
            .sum();
        (ans % 1000_000_007) as _
    }
}
// @lc code=end

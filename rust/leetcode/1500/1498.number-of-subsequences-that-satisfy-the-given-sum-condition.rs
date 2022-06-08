/*
 * @lc app=leetcode id=1498 lang=rust
 *
 * [1498] Number of Subsequences That Satisfy the Given Sum Condition
 */

// @lc code=start
impl Solution {
    pub fn num_subseq(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let p = 1000_000_007;
        let mut f = vec![0; n];
        f[0] = 1;
        for i in 1..n {
            f[i] = ((f[i - 1] as i64 * 2) % p as i64) as i32;
        }
        let mut nums = nums;
        nums.sort();
        let mut ans = 0;
        for (i, v) in nums.iter().enumerate() {
            if v * 2 > target {
                break;
            }
            let mv = target - v;
            let j = match nums.binary_search(&mv) {
                Ok(mut j) => {
                    while j < nums.len() && nums[j] <= mv {
                        j += 1;
                    }
                    j
                }
                Err(j) => j,
            };
            ans += if j > i { f[j - i - 1] } else { 0 };
            ans %= p;
        }
        ans % p
    }
}
// @lc code=end

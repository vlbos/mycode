/*
 * @lc app=leetcode id=698 lang=rust
 *
 * [698] Partition to K Equal Sum Subsets
 */

// @lc code=start
impl Solution {
    pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
        if k == 1 {
            return true;
        }
        let sum = nums.iter().sum::<i32>();
        if sum % k != 0 {
            return false;
        }
        let avg = sum / k;
        if nums.iter().any(|&x| x > avg) {
            return false;
        }
        let mut nums = nums;
        nums.sort();
        let len = nums.len();
        let size = 1 << len;
        let mut dp = vec![false; size];
        let mut currsum = vec![0; size];
        dp[0] = true;
        for i in 0..size {
            if !dp[i] {
                continue;
            }

            for j in 0..len {
                if i & (1 << j) != 0 {
                    continue;
                }
                let next = i | (1 << j);
                if dp[next] {
                    continue;
                }
                if (currsum[i] % avg) + nums[j] <= avg {
                    currsum[next] = currsum[i] + nums[j];
                    dp[next] = true;
                } else {
                    break;
                }
            }
        }
        dp[size - 1]
    }
}
// @lc code=end

/*
 * @lc app=leetcode id=1681 lang=rust
 *
 * [1681] Minimum Incompatibility
 */

// @lc code=start
impl Solution {
    pub fn minimum_incompatibility(nums: Vec<i32>, k: i32) -> i32 {
        fn f(mask: i32, pre: usize, nums: &Vec<i32>, per: i32, dp: &mut Vec<i32>) -> i32 {
            let maski = mask as usize;
            let lowbit = |x: i32| x & (-x);
            if mask == 0 {
                return 0;
            }
            let n = nums.len();
            if dp[maski * n + pre] != -1 {
                return dp[maski * n + pre];
            }
            let cnt = mask.count_ones() as i32;
            let rem = cnt % per;
            let mut ans = i32::MAX/2;
            if rem == 0 {
                ans = f((mask ^ lowbit(mask)), mask.trailing_zeros() as usize , nums, per, dp);
                for pre in 0..n {
                    dp[maski * n + pre] = ans;
                }
            } else {
                if (mask >> (pre + 1)).count_ones() as i32 >= rem {
                    for p in (pre + 1)..n {
                        if (mask & (1 << p)) > 0 && nums[p] > nums[pre] {
                            ans = ans.min(f((mask ^ (1 << p)), p, nums, per, dp) + nums[p] - nums[pre]);
                        }
                    }
                }
                dp[maski * n + pre] = ans;
            }
            ans
        }
        let n = nums.len();
        let mut nums = nums;
        nums.sort();
        if k == 1 {
            let mut nums_set = nums.clone();
            nums_set.dedup();
            if nums_set.len() < nums.len() {
                return -1;
            }
            return nums_set.last().unwrap() - nums_set.first().unwrap();
        }
        if n as i32 == k {
            return 0;
        }
        let per = n as i32 / k;
             let nn= (1 << n) * n;
        let mut dp = vec![-1; nn];
        let n16=((1<<16)*16);
        if n16>nn{
            dp.extend(vec![0; n16-nn]);
        }
        let ans = f((1 << n) - 1, 0, &nums, per, &mut dp);
        if ans >= 10000 {
            -1
        } else {
            ans
        }
    }
}
// @lc code=end

/*
 * @lc app=leetcode id=1755 lang=rust
 *
 * [1755] Closest Subsequence Sum
 */

// @lc code=start
impl Solution {
    pub fn min_abs_difference(nums: Vec<i32>, goal: i32) -> i32 {
        let n = nums.len();
        let half = n / 2;
        let (ls, rs) = (half, n - half);
        let (ls1, rs1) = (1 << ls, 1 << rs);
        let mut lsum = vec![0; ls1];
        for i in 1..ls1 {
            for j in 0..ls {
                if i & (1 << j) > 0 {
                    lsum[i] = lsum[i - (1 << j)] + nums[j];
                    break;
                }
            }
        }
        let mut rsum = vec![0; rs1];
        for i in 1..rs1 {
            for j in 0..rs {
                if i & (1 << j) > 0 {
                    rsum[i] = rsum[i - (1 << j)] + nums[ls + j];
                    break;
                }
            }
        }
        lsum.sort();
        rsum.sort();
        let mut ans = lsum
            .iter()
            .chain(rsum.iter())
            .map(|x| (*x - goal).abs())
            .min()
            .unwrap();
        let (mut i, mut j) = (0, rsum.len() - 1);
        while i < lsum.len() && j >= 0 {
            let s = lsum[i] + rsum[j];
            ans = ans.min((s - goal).abs());
            if s > goal {
                if j == 0 {
                    break;
                }
                j -= 1;
            } else {
                i += 1;
            }
        }
        ans
    }
}
// @lc code=end

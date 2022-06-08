/*
 * @lc app=leetcode id=1494 lang=rust
 *
 * [1494] Parallel Courses II
 */

// @lc code=start
impl Solution {
    pub fn min_number_of_semesters(n: i32, relations: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut pre_req = vec![0; n as usize];
        for r in &relations {
            pre_req[r[1] as usize - 1] |= 1 << (r[0] - 1);
        }
        let n1 = 1 << n;
        let mut set_prereq = vec![0; n1];
        let mut valid = vec![false; n1];
        for mask in 0..n1 {
            if mask.count_ones() <= k as u32{
                for i in 0..n {
                    if mask & (1 << i) > 0 {
                        set_prereq[mask] |= pre_req[i as usize];
                    }
                }
                valid[mask] = set_prereq[mask] & mask == 0;
            }
        }
        let mut dp = vec![i32::MAX / 2; n1];
        dp[0] = 0;
        for mask in 0..n1 {
            let mut subset = mask;
            while subset > 0 {
                if valid[subset] && set_prereq[subset] & mask == set_prereq[subset] {
                    dp[mask] = dp[mask].min(dp[mask ^ subset] + 1);
                }
                subset = (subset - 1) & mask;
            }
        }
        dp[n1 - 1]
    }
}
// @lc code=end

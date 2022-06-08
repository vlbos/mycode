/*
 * @lc app=leetcode id=1601 lang=rust
 *
 * [1601] Maximum Number of Achievable Transfer Requests
 */

// @lc code=start
impl Solution {
    pub fn maximum_requests(n: i32, requests: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        for mask in 0..(1 << requests.len()) {
            let cnt = (mask as i32).count_ones();
            if cnt <= ans {
                continue;
            }
            let mut delta = vec![0; n as usize];
            for (i, (x, y)) in requests
                .iter()
                .map(|x| (x[0] as usize, x[1] as usize))
                .enumerate()
            {
                if mask & (1 << i) > 0 {
                    delta[x] += 1;
                    delta[y] -= 1;
                }
            }
            if delta.iter().all(|x| *x == 0) {
                ans = cnt;
            }
        }

        ans as _
    }
}
// @lc code=end

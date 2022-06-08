/*
 * @lc app=leetcode id=2035 lang=rust
 *
 * [2035] Partition Array Into Two Arrays to Minimize Sum Difference
 */

// @lc code=start
impl Solution {
    pub fn minimum_difference(nums: Vec<i32>) -> i32 {
        let n = nums.len() / 2;
        let mut res = vec![Vec::new(); n + 1];
        for i in 0..(1 << n) {
            let (mut sum, mut cnt) = (0, 0);
            for (j, &v) in nums[..n].iter().enumerate() {
                if i & (1 << j) == 0 {
                    sum -= v;
                } else {
                    cnt += 1;
                    sum += v;
                }
            }
            res[cnt].push(sum);
        }
        res.iter_mut().for_each(|x| x.sort());
        let mut ans = i32::MAX;
        for i in 0..(1 << n) {
            let (mut sum, mut cnt) = (0, 0);
            for (j, &v) in nums[n..].iter().enumerate() {
                if i & (1 << j) == 0 {
                    sum -= v;
                } else {
                    cnt += 1;
                    sum += v;
                }
            }
            match res[cnt].binary_search(&sum) {
                Ok(k) | Err(k) if k < res[cnt].len() => {
                    ans = ans.min(res[cnt][k] - sum);
                    if k > 0 {
                        ans = ans.min(sum - res[cnt][k - 1]);
                    }
                }
                _ => {}
            }
        }
        ans
    }
}
// @lc code=end

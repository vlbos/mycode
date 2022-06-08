/*
 * @lc app=leetcode id=164 lang=rust
 *
 * [164] Maximum Gap
 */

// @lc code=start
impl Solution {
    pub fn maximum_gap(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n < 2 {
            return 0;
        }
        let min_val = *nums.iter().min().unwrap();
        let max_val = *nums.iter().max().unwrap();
        let mm = (max_val - min_val) as usize;
        let d = 1usize.max(mm / (n - 1));
        let bucket_len = mm / d + 1;
        let mut bucket = vec![vec![-1, -1]; bucket_len];
        for &v in &nums {
            let idx = (v - min_val) as usize / d;
            if bucket[idx][0] == -1 {
                bucket[idx][0] = v;
                bucket[idx][1] = v;
            } else {
                bucket[idx][0] = v.min(bucket[idx][0]);
                bucket[idx][1] = v.max(bucket[idx][1]);
            }
        }
        let mut ans = 0;
        let mut pre = bucket_len;
        for (i, b) in bucket.iter().enumerate() {
            if b[0] == -1 {
                continue;
            }
            if pre != bucket_len {
                ans = ans.max(b[0] - bucket[pre][1]);
            }
            pre = i;
        }
        ans
    }
}
// @lc code=end

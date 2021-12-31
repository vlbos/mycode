/*
 * @lc app=leetcode id=128 lang=rust
 *
 * [128] Longest Consecutive Sequence
 */

// @lc code=start
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return nums.len() as i32;
        }
        let min = *(nums.iter().min().unwrap());
        let max = *(nums.iter().max().unwrap());
        let mut v = vec![false; (max - min + 1) as usize];
        for n in &nums {
            v[(*n - min) as usize] = true;
        }
        let mut ans = 0;
        let mut cnt = 0;
        for n in &v {
            if *n {
                cnt += 1;
            } else {
                if cnt > ans {
                    ans = cnt;
                }
                cnt = 0;
            }
        }
        if cnt > ans {
            ans = cnt;
        }
        ans
    }
}
// @lc code=end

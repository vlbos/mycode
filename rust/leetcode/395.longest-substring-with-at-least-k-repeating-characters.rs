/*
 * @lc app=leetcode id=395 lang=rust
 *
 * [395] Longest Substring with At Least K Repeating Characters
 */

// @lc code=start
impl Solution {
    pub fn longest_substring(s: String, k: i32) -> i32 {
        let mut sv = s.chars().collect::<Vec<char>>();
        let n = sv.len();
        let mut ans = 0;
        for t in 1..=26 {
            let mut cnt = vec![0; 26];
            let mut l = 0;
            let mut r = 0;
            let mut tot = 0;
            let mut less = 0;
            while r < n {
                let j = (sv[r] as u8 - b'a') as usize;
                cnt[j] += 1;
                if cnt[j] == 1 {
                    tot += 1;
                    less += 1;
                }
                if cnt[j] == k {
                    less -= 1;
                }
                while tot > t {
                    let j = (sv[l] as u8 - b'a') as usize;
                    cnt[j] -= 1;
                    if cnt[j] == 0 {
                        tot -= 1;
                        less -= 1;
                    }
                    if cnt[j] == k - 1 {
                        less += 1;
                    }

                    l += 1;
                }
                if less == 0 {
                    ans = ans.max(r - l + 1);
                }
                r += 1;
            }
        }
        ans as i32
    }
}
// @lc code=end

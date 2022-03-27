/*
 * @lc app=leetcode id=2086 lang=rust
 *
 * [2086] Minimum Number of Buckets Required to Collect Rainwater from Houses
 */

// @lc code=start
impl Solution {
    pub fn minimum_buckets(street: String) -> i32 {
        let n = street.len();
        let bs = street.as_bytes();
        let mut ans = 0;
        let mut i = 0;
        while i < n {
            if bs[i] == b'H' {
                if i + 1 < n && bs[i + 1] == b'.' {
                    ans += 1;
                    i += 2;
                } else if i > 0 && bs[i - 1] == b'.' {
                    ans += 1;
                } else {
                    return -1;
                }
            }
            i += 1;
        }
        ans
    }
}
// @lc code=end

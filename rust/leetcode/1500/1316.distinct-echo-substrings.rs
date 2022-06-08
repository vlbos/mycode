/*
 * @lc app=leetcode id=1316 lang=rust
 *
 * [1316] Distinct Echo Substrings
 */

// @lc code=start
impl Solution {
    pub fn distinct_echo_substrings(text: String) -> i32 {
       let n = text.len();
        let base = 31;
        let mut pre = vec![0; n + 1];
        let mut mul = vec![0; n + 1];
        mul[0] = 1;
        let p = 1_000_000_007;
        let tb = text.as_bytes();
        for i in 1..=n {
            pre[i] = (pre[i - 1] * base + tb[i - 1] as i64) % p;
            mul[i] = mul[i - 1] * base % p;
        }
        let mut seen = vec![std::collections::HashSet::new(); n];
        let mut ans = 0;
        let get_hash=|l:usize,r:usize|{
            (pre[r+1]+p-(pre[l]*mul[r+1-l])%p)%p
        };
        for i in 0..n {
            for j in i + 1..n {
                let l = j - i;
                if j + l > n {
                    continue;
                }
                let hash_left = get_hash(i, j - 1);
                if !seen[l - 1].contains(&hash_left) && hash_left == get_hash(j, j + l - 1) {
                    ans += 1;
                    seen[l - 1].insert(hash_left);
                }
            }
        }
        ans
    }
}
// @lc code=end

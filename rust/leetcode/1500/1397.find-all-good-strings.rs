/*
 * @lc app=leetcode id=1397 lang=rust
 *
 * [1397] Find All Good Strings
 */

// @lc code=start
impl Solution {
    pub fn find_good_strings(n: i32, s1: String, s2: String, evil: String) -> i32 {
        let m = evil.len();
        let mut fail = vec![0; m];
        let be = evil.as_bytes();
        for i in 1..m {
            let mut j = fail[i - 1];
            while j > 0 && be[j] != be[i] {
                j = fail[j - 1];
            }
            if be[j] == be[i] {
                fail[i] = j + 1;
            }
        }
        fn get_trans(
            mut stats: usize,
            ch: u8,
            be: &[u8],
            fail: &Vec<usize>,
            trans: &mut Vec<Vec<i32>>,
        ) -> i32 {
            let bi = (ch - b'a') as usize;
            if trans[stats][bi] != -1 {
                return trans[stats][bi];
            }
            while stats > 0 && be[stats] != ch {
                stats = fail[stats - 1];
            }
            trans[stats][bi] = if be[stats] == ch { stats as i32 + 1 } else { 0 };
            trans[stats][bi]
        }
        fn dfs(
            pos: usize,
            stats: usize,
            bound: usize,
            bs1: &[u8],
            bs2: &[u8],
            be: &[u8],
            fail: &Vec<usize>,
            trans: &mut Vec<Vec<i32>>,
            f: &mut Vec<Vec<Vec<i32>>>,
        ) -> i32 {
            if stats == be.len() {
                return 0;
            }
            if pos == bs1.len() {
                return 1;
            }
            if f[pos][stats][bound] != -1 {
                return f[pos][stats][bound];
            }
            f[pos][stats][bound] = 0;
            let (l, r) = (
                if bound & 1 > 0 { bs1[pos] } else { b'a' },
                if bound & 2 > 0 { bs2[pos] } else { b'z' },
            );
            for ch in l..=r {
                let nxt_stats = get_trans(stats, ch, be, fail, trans);
                let nxt_bound = (if bound & 1 > 0 && ch == bs1[pos] {
                    1
                } else {
                    0
                }) ^ (if bound & 2 > 0 && ch == bs2[pos] {
                    2
                } else {
                    0
                });
                f[pos][stats][bound] += dfs(
                    pos + 1,
                    nxt_stats as usize,
                    nxt_bound as usize,
                    bs1,
                    bs2,
                    be,
                    fail,
                    trans,
                    f,
                );
                f[pos][stats][bound] %= 1_000_000_007;
            }
            f[pos][stats][bound]
        }
        let mut f = vec![vec![vec![-1; 4]; 50]; 500];
        let mut trans = vec![vec![-1; 26]; 50];
        let bs1 = s1.as_bytes();
        let bs2 = s2.as_bytes();
        dfs(0, 0, 3, bs1, bs2, be, &fail, &mut trans, &mut f)
    }
}
// @lc code=end

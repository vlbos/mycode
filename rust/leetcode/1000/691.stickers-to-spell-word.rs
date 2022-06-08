/*
 * @lc app=leetcode id=691 lang=rust
 *
 * [691] Stickers to Spell Word
 */

// @lc code=start
impl Solution {
    pub fn min_stickers(stickers: Vec<String>, target: String) -> i32 {
        let mut dp = vec![i32::MAX; 1 << 15];
        let (n, m) = (stickers.len(), target.len());
        let mut cnt = vec![vec![0; 26]; n];
        let mut can = vec![Vec::new(); 26];
        let bt = target.as_bytes();
        for (i, sticker) in stickers.iter().enumerate() {
            for b in sticker.bytes() {
                let d = (b - b'a') as usize;
                cnt[i][d] += 1;
                if can[d].is_empty() || can[d][can[d].len() - 1] != i {
                    can[d].push(i);
                }
            }
        }
        dp[0] = 0;
        for i in 0..(1 << m) - 1 {
            if dp[i] == i32::MAX {
                continue;
            }
            let mut d = 0;
            for j in 0..m {
                if i & (1 << j) == 0 {
                    d = j;
                    break;
                }
            }
            d = (bt[d] - b'a') as usize;
            for &k in &can[d] {
                let mut nxt = i;
                let mut left = cnt[k].clone();
                for j in 0..m {
                    if nxt & (1 << j) != 0 {
                        continue;
                    }
                    if left[(bt[j] - b'a') as usize] > 0 {
                        nxt += (1 << j);
                        left[(bt[j] - b'a') as usize] -= 1;
                    }
                }
                dp[nxt] = dp[nxt].min(dp[i] + 1);
            }
        }
        if dp[(1 << m) - 1] == i32::MAX {
            -1
        } else {
            dp[(1 << m) - 1]
        }
    }
}
// @lc code=end

/*
 * @lc app=leetcode id=943 lang=rust
 *
 * [943] Find the Shortest Superstring
 */

// @lc code=start
impl Solution {
    pub fn shortest_superstring(words: Vec<String>) -> String {
       let n = words.len();
        let mut overlaps = vec![vec![0; n]; n];
        for (i, x) in words.iter().enumerate() {
            for (j, y) in words.iter().enumerate() {
                if i == j {
                    continue;
                }
                for k in (0..=x.len().min(y.len())).rev() {
                    if words[i].ends_with(&words[j][..k]) {
                        overlaps[i][j] = k;
                        break;
                    }
                }
            }
        }
        let n2 = 1 << n;
        let mut dp = vec![vec![0; n]; n2];
        let mut parent = vec![vec![-1; n]; n2];
        for mask in 0..n2 {
            for bit in 0..n {
                if (mask >> bit) & 1 == 0 {
                    continue;
                }
                let pmask = mask ^ (1 << bit);
                if pmask == 0 {
                    continue;
                }
                for i in 0..n {
                    if (pmask >> i) & 1 == 0 {
                        continue;
                    }
                    let val = dp[pmask][i] + overlaps[i][bit];
                    if val > dp[mask][bit] {
                        dp[mask][bit] = val;
                        parent[mask][bit] = i as i32;
                    }
                }
            }
        }
        let mut perm = Vec::new();
        let mut t = 0;
        let mut mask = n2 - 1;
        let mut i = dp[mask].iter().enumerate().max_by_key(|x| x.1).unwrap().0 as i32;
        while i != -1 {
            perm.push(i);

            let i2 = parent[mask][i as usize];
            mask ^= 1 << i;
            i = i2;
        }
        perm.reverse();
        let mut seen = vec![false; n];
        for &i in &perm {
            seen[i as usize] = true;
        }
        perm.extend(
            (0..n as i32)
                .filter(|i| !seen[*i as usize])
                .collect::<Vec<i32>>(),
        );
        let mut ans = vec![words[perm[0] as usize].clone()];
        for i in 1..perm.len() {
            let overlap = overlaps[perm[i - 1] as usize][perm[i] as usize];
            ans.push(words[perm[i] as usize][overlap..].to_string());
        }
        ans.concat()
    }
}
// @lc code=end

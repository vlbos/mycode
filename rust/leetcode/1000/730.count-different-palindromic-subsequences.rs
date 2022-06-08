/*
 * @lc app=leetcode id=730 lang=rust
 *
 * [730] Count Different Palindromic Subsequences
 */

// @lc code=start
impl Solution {
    pub fn count_palindromic_subsequences(s: String) -> i32 {
        let n = s.len();
        let a = s.bytes().map(|x| x - b'a').collect::<Vec<u8>>();
        let mut prv = vec![Vec::new(); n];
        let mut nxt = vec![Vec::new(); n];
        let mut last = vec![None; 4];
        for i in 0..n {
            last[a[i] as usize] = Some(i);
            prv[i] = last.clone();
        }
        last = vec![None; 4];
        for i in (0..n).rev() {
            last[a[i] as usize] = Some(i);
            nxt[i] = last.clone();
        }
        let mut memo = vec![vec![None; n]; n];
        fn dp(
            i: usize,
            j: usize,
            prv: &Vec<Vec<Option<usize>>>,
            nxt: &Vec<Vec<Option<usize>>>,
            memo: &mut Vec<Vec<Option<i32>>>,
        ) -> i32 {
                    let p = 1000_000_007;

            if let Some(v) = memo[i][j] {
                return v ;
            }
            let mut ans = 1;
            if i <= j {
                for k in 0..4 {
                    let (i0, j0) = (nxt[i][k], prv[j][k]);
                    if i0.is_none() || j0.is_none() {
                        continue;
                    }
                    let (i0, j0) = (i0.unwrap(), j0.unwrap());
                    if i <= i0 && i0 <= j {
                        ans += 1;
                    }
                    if i0 < j0 {
                        ans += dp(i0 + 1, j0 - 1,prv,nxt,memo);
                    }
                    if ans >= p {
                        ans -= p;
                    }
                }
            }
            memo[i][j] = Some(ans);
            ans
        }
        dp(0, n - 1,&prv,&nxt,&mut memo)  - 1
    }
}
// @lc code=end

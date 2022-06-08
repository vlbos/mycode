/*
 * @lc app=leetcode id=2060 lang=rust
 *
 * [2060] Check if an Original String Exists Given Two Encoded Strings
 */

// @lc code=start
impl Solution {
    pub fn possibly_equals(s1: String, s2: String) -> bool {
        let (m, n) = (s1.len(), s2.len());
        let mut f = vec![vec![vec![vec![None; 1000]; 2]; n + 1]; m + 1];
        fn dfs(
            i: usize,
            j: usize,
            which: usize,
            rest: usize,
            s1: &String,
            s2: &String,
            f: &mut Vec<Vec<Vec<Vec<Option<bool>>>>>,
        ) -> bool {
            let (m, n) = (s1.len(), s2.len());
            if let Some(r) =f[i][j][which][rest]{
                return r;
            }
            if which == 0 {
                f[i][j][which][rest] = Some(if j == n {
                    (i == m && rest == 0)
                } else if s2.chars().nth(j).unwrap().is_ascii_alphabetic() {
                    if rest == 0 && i != m && s1.chars().nth(i).unwrap().is_ascii_alphabetic() {
                        if s1.chars().nth(i).unwrap() == s2.chars().nth(j).unwrap() {
                            dfs(i + 1, j + 1, 0, 0, s1, s2, f)
                        } else {
                            false
                        }
                    } else {
                        if rest > 0 {
                            dfs(i, j + 1, 0, rest - 1, s1, s2, f)
                        } else {
                            dfs(i, j + 1, 1, 1, s1, s2, f)
                        }
                    }
                } else {
                    let (mut x, mut k) = (0, j);
                    let mut ans = false;
                    while k < n && s2.chars().nth(k).unwrap().is_ascii_digit() {
                        x = x * 10 + (s2.as_bytes()[k] - b'0') as usize;
                        if rest >= x && dfs(i, k + 1, 0, rest - x, s1, s2, f)
                            || (rest < x && dfs(i, k + 1, 1, x - rest, s1, s2, f))
                        {
                            ans = true;
                            break;
                        }
                        k += 1;
                    }
                    ans
                });
                return f[i][j][which][rest].unwrap();
            }
            f[i][j][which][rest] = Some(if i == m {
                (j == n && rest == 0)
            } else if s1.chars().nth(i).unwrap().is_ascii_alphabetic() {
                if rest == 0 && j != n && s2.chars().nth(j).unwrap().is_ascii_alphabetic() {
                    if s1.chars().nth(i).unwrap() == s2.chars().nth(j).unwrap() {
                        dfs(i + 1, j + 1, 0, 0, s1, s2, f)
                    } else {
                        false
                    }
                } else {
                    if rest > 0 {
                        dfs(i + 1, j, 1, rest - 1, s1, s2, f)
                    } else {
                        dfs(i+1, j , 0, 1, s1, s2, f)
                    }
                }
            } else {
                let (mut x, mut k) = (0, i);
                let mut ans = false;
                while k < m && s1.chars().nth(k).unwrap().is_ascii_digit() {
                    x = x * 10 + (s1.as_bytes()[k] - b'0') as usize;
                    if rest >= x && dfs(k + 1, j, 1, rest - x, s1, s2, f)
                        || (rest < x && dfs(k + 1, j, 0, x - rest, s1, s2, f))
                    {
                        ans = true;
                        break;
                    }
                    k += 1;
                }
                ans
            });
            f[i][j][which][rest].unwrap()
        }
        dfs(0, 0, 0, 0, &s1, &s2, &mut f)
    }
}
// @lc code=end

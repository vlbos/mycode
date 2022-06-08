/*
 * @lc app=leetcode id=2018 lang=rust
 *
 * [2018] Check if Word Can Be Placed In Crossword
 */

// @lc code=start
impl Solution {
    pub fn place_word_in_crossword(board: Vec<Vec<char>>, word: String) -> bool {
       let (m, n, k) = (board.len(), board[0].len(), word.len());
        let w = word.as_bytes();
        for r in &board {
            let mut j = 0;
            while j < n {
                if r[j] == '#' {
                    j += 1;
                    continue;
                }
                let (j0, mut ok1, mut ok2) = (j, true, true);
                while j < n && r[j] != '#' {
                    if j - j0 >= k || r[j] != ' ' && (r[j] as u8) != w[j - j0] {
                        ok1 = false;
                    }
                    if j - j0 >= k || r[j] != ' ' && (r[j] as u8) != w[k - 1 - j + j0] {
                        ok2 = false;
                    }
                    j += 1;
                }
                if (ok1 || ok2) && j - j0 == k {
                    return true;
                }
                j += 1;
            }
        }

        for j in 0..n {
            let mut i = 0;
            while i < m {
                if board[i][j] == '#' {
                    i += 1;
                    continue;
                }
                let (i0,mut ok1, mut ok2) = (i, true, true);
                while i < m && board[i][j] != '#' {
                    if i - i0 >= k || board[i][j] != ' ' && (board[i][j] as u8) != w[i - i0] {
                        ok1 = false;
                    }
                    if i - i0 >= k || board[i][j] != ' ' && (board[i][j] as u8) != w[k - 1 - i + i0]
                    {
                        ok2 = false;
                    }
                    i += 1;
                }
                if (ok1 || ok2) && i - i0 == k {
                    return true;
                }
                i += 1;
            }
        }
        false
    }
}
// @lc code=end

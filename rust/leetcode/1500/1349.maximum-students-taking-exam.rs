/*
 * @lc app=leetcode id=1349 lang=rust
 *
 * [1349] Maximum Students Taking Exam
 */

// @lc code=start
impl Solution {
    pub fn max_students(seats: Vec<Vec<char>>) -> i32 {
        let mut memo = vec![vec![-1; 1 << 8]; 8];
        let compress = |row: &Vec<char>| -> usize {
            let mut ans = 0;
            for &c in row {
                ans <<= 1;
                if c == '.' {
                    ans += 1;
                }
            }
            ans
        };
        fn f(
            x: usize,
            row_num: usize,
            width: usize,
            memo: &mut Vec<Vec<i32>>,
            compressed_seats: &Vec<usize>,
        ) -> i32 {
            if memo[row_num][x] != -1 {
                return memo[row_num][x];
            }
            let mut ans = 0;
            for scheme in 0..(1 << width) {
                if scheme & (!x) > 0 || scheme & (scheme << 1) > 0 {
                    continue;
                }
                let mut curans = 0;
                for j in 0..width {
                    if (1 << j) & scheme > 0 {
                        curans += 1;
                    }
                }
                if row_num == compressed_seats.len() - 1 {
                    ans = ans.max(curans);
                } else {
                    let mut next_seat = compressed_seats[row_num + 1];
                    next_seat &= !(scheme << 1);
                    next_seat &= !(scheme >> 1);
                    ans =
                        ans.max(curans + f(next_seat, row_num + 1, width, memo, compressed_seats));
                }
            }
            memo[row_num][x] = ans;
            ans
        }
        let m = seats[0].len();
        let compressed_seats: Vec<usize> = seats.iter().map(|x| compress(x)).collect();
        f(compressed_seats[0] , 0, m, &mut memo, &compressed_seats)
    }
}
// @lc code=end

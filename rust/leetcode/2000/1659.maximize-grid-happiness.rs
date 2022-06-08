/*
 * @lc app=leetcode id=1659 lang=rust
 *
 * [1659] Maximize Grid Happiness
 */

// @lc code=start
impl Solution {
    pub fn get_max_grid_happiness(
        m: i32,
        n: i32,
        introverts_count: i32,
        extroverts_count: i32,
    ) -> i32 {
       let mut mask_span = vec![vec![0; 6]; 729];
        let mut dp = vec![vec![vec![vec![-1; 7]; 7]; 729]; 25];
        let mut truncate = vec![vec![0; 3]; 729];
   
        let n = n as usize;
        let n3 = 3usize.pow(n as u32);
        let highest = n3 / 3;
        for mask in 0..n3 {
            let mut mask_tmp = mask;
            for i in 0..n {
                mask_span[mask][n - i - 1] = mask_tmp % 3;
                mask_tmp /= 3;
            }
            for i in 0..3 {
                truncate[mask][i] = mask % highest * 3 + i;
            }
        }
        fn dfs(
            pos: usize,
            borderline: usize,
            introverts_count: usize,
            extroverts_count: usize,
            m: usize,
            n: usize,
            dp: &mut Vec<Vec<Vec<Vec<i32>>>>,
            mask_span: &Vec<Vec<usize>>,
            truncate: &Vec<Vec<usize>>,
        ) -> i32 {
     let calc = |x: i32, y: i32| match (x, y) {
            (0, _) | (_, 0) => 0,
            (1, 1) => -60,
            (2, 2) => 40,
            _ => -10,
        };
            if pos == m * n || introverts_count + extroverts_count == 0 {
                return 0;
            }
            if dp[pos][borderline][introverts_count][extroverts_count] != -1 {
                return dp[pos][borderline][introverts_count][extroverts_count];
            }
            let (x, y) = (pos / n, pos % n);
            let mut best = dfs(
                pos + 1,
                truncate[borderline][0] ,
                introverts_count,
                extroverts_count,
                m,
                n,
                dp,
                mask_span,
                truncate,
            );
            if introverts_count > 0 {
                best = best.max(
                    120 + calc(1, mask_span[borderline][0]  as i32)
                        + if y == 0 {
                            0
                        } else {
                            calc(1, mask_span[borderline][n - 1]  as i32)
                        }
                        + dfs(
                            pos + 1,
                            truncate[borderline][1],
                            introverts_count - 1,
                            extroverts_count,
                            m,
                            n,
                            dp,
                            mask_span,
                            truncate,
                        ),
                );
            }
            if extroverts_count > 0 {
                best = best.max(
                    40 + calc(2, mask_span[borderline][0] as i32)
                        + if y == 0 {
                            0
                        } else {
                            calc(2, mask_span[borderline][n - 1]  as i32)
                        }
                        + dfs(
                            pos + 1,
                            truncate[borderline][2],
                            introverts_count,
                            extroverts_count - 1,
                            m,
                            n,
                            dp,
                            mask_span,
                            truncate,
                        ),
                );
            }
            dp[pos][borderline][introverts_count][extroverts_count] = best;
            best
        }
        dfs(
            0,
            0,
            introverts_count as usize,
            extroverts_count as usize,
            m as usize,
            n as usize,
            &mut dp,
            &mask_span,
            &truncate,
        )
    }
}
// @lc code=end

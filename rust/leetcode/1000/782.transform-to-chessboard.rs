/*
 * @lc app=leetcode id=782 lang=rust
 *
 * [782] Transform to Chessboard
 */

// @lc code=start
impl Solution {
    pub fn moves_to_chessboard(board: Vec<Vec<i32>>) -> i32 {
        let n = board.len();
        use std::collections::HashMap;

        let analyse_count = |count: &HashMap<Vec<i32>, i32>| -> i32 {
            if count.len() != 2 {
                return -1;
            }
            let mut values: Vec<i32> = count.values().cloned().collect();
            values.sort();
            let nn = n as i32;
            if values != vec![nn / 2, (nn + 1) / 2] {
                return -1;
            }
            let keys: Vec<Vec<i32>> = count.keys().cloned().collect();
            if keys[0].iter().zip(&keys[1]).any(|(a, b)| a ^ b==0) {
                return -1;
            }
            let ones = keys[0].iter().filter(|&x| *x > 0).count();
            let mut cand = i32::MAX;
            let mut mask01 = vec![vec![0, 1]; n / 2].iter().flatten().cloned().collect::<Vec<i32>>();
            let mut mask10 = vec![vec![1, 0]; n / 2].iter().flatten().cloned().collect::<Vec<i32>>();
            if n % 2 > 0 {
                mask01.push(0);
                mask10.push(1);
            }
            if n % 2 == 0 || ones * 2 < n {
                cand = cand.min(keys[0].iter().zip(mask01).filter(|(&a, b)| a ^ b > 0).count() as i32/ 2);
            }
            if n % 2 == 0 || ones * 2 > n {
                cand = cand.min(keys[0].iter().zip(mask10).filter(|(&a, b)| a ^ b > 0).count() as i32 / 2);
            }
            cand
        };
        let mut rm = HashMap::new();
        board
            .iter()
            .for_each(|row| *rm.entry(row.clone()).or_insert(0) += 1);
        let k1 = analyse_count(&rm);
        if k1 == -1 {
            return -1;
        }

        let mut cm = HashMap::new();
        for j in 0..n {
            let mut code = Vec::new();
            for i in 0..n {
                code.push(board[i][j]);
            }
            *cm.entry(code).or_insert(0) += 1;
        }

        let k2 = analyse_count(&cm);
        if k2 == -1 {
            return -1;
        }
        k1 + k2
    }
}
// @lc code=end

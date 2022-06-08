/*
 * @lc app=leetcode id=2056 lang=rust
 *
 * [2056] Number of Valid Move Combinations On Chessboard
 */

// @lc code=start
impl Solution {
    pub fn count_combinations(pieces: Vec<String>, positions: Vec<Vec<i32>>) -> i32 {
       let dxy = [
            [1, 0],
            [-1, 0],
            [0, 1],
            [0, -1],
            [1, 1],
            [-1, -1],
            [-1, 1],
            [1, -1],
        ];
        let mut positions: Vec<Vec<i32>> = positions
            .into_iter()
            .map(|x| vec![x[0] - 1, x[1] - 1])
            .collect();
        let n = positions.len();
        let mut m = vec![(0, 0); n];

        let mut ans = 0;
        fn dfs(
            i: usize,
            pieces: &Vec<String>,
            positions: &Vec<Vec<i32>>,
            dxy: &[[i32; 2]],
            m: &mut Vec<(i32, i32)>,
            ans: &mut i32,
        ) {
            let n = positions.len();
            let sim = || {
                let mut board = vec![vec![0; 8]; 8];
                let mut mv = vec![(0, 0); n];
                let mut cur_pos = vec![vec![0; 2]; n];
                for (i, pos) in positions.iter().enumerate() {
                    mv[i] = m[i];
                    cur_pos[i] = pos.clone();
                }
                loop {
                    let mut moved = false;
                    for (i, pos) in cur_pos.iter_mut().enumerate() {
                        if mv[i].1 > 0 {
                            moved = true;
                            mv[i].1 -= 1;
                            pos[0] += dxy[mv[i].0 as usize][0] ;
                            pos[1] += dxy[mv[i].0  as usize][1];
                        }
                    }
                    if !moved {
                        break;
                    }
                    for pos in &cur_pos {
                        board[pos[0] as usize][pos[1] as usize] += 1;
                        if board[pos[0] as usize][pos[1] as usize] > 1 {
                            return 0;
                        }
                    }
                    for pos in &cur_pos {
                        board[pos[0] as usize][pos[1] as usize] = 0;
                    }
                }
                1
            };
            if i == pieces.len() {
                *ans += sim();
                return;
            }
            let (mind, maxd) = match pieces[i].as_str() {
                "rook" => (0, 3),
                "queen" => (0, 7),
                _ => (4, 7),
            };
            for d in mind..=maxd {
                for l in 1..=8 {
                    if positions[i][0] + l * dxy[d][0] >= 0
                        && positions[i][0] + l * dxy[d][0] < 8
                        && positions[i][1] + l *dxy[d][1] >= 0
                        && positions[i][1] + l * dxy[d][1] < 8
                    {
                        m[i] = (d as i32, l);
                        dfs(i + 1,  pieces, positions,dxy,  m,ans);
                    }
                }
            }
            m[i] = (0, 0);
            dfs(i + 1, pieces, positions,dxy,  m, ans);
        }
        dfs(0, &pieces, &positions,&dxy, &mut m, &mut ans);
        ans
    }
}
// @lc code=end

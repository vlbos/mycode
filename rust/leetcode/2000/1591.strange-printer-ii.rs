/*
 * @lc app=leetcode id=1591 lang=rust
 *
 * [1591] Strange Printer II
 */

// @lc code=start
impl Solution {
    pub fn is_printable(target_grid: Vec<Vec<i32>>) -> bool {
        let mut range = vec![vec![usize::MAX, usize::MIN, usize::MAX, usize::MIN]; 61];
        let mut existed = vec![false; 61];
        for (i, row) in target_grid.iter().enumerate() {
            for (j, &col) in row.iter().enumerate() {
                for (k, mut v) in range[col as usize].iter_mut().enumerate() {
                    let k2 = k / 2;
                    let ij = vec![i, j];
                    *v = if k % 2 == 0 {
                        (*v).min(ij[k2])
                    } else {
                        (*v).max(ij[k2])
                    };
                }
                existed[col as usize] = true;
            }
        }
        let mut cnt = existed.iter().filter(|&x| *x).count();
        let mut target_grid = target_grid;
        loop {
            let mut found = false;
            for (k, ev) in existed.iter_mut().enumerate().filter(|x| *x.1) {
                let ri = &range[k];
                let mut macthed = true;
                for i in (ri[0]..=ri[1]) {
                    for j in (ri[2]..=ri[3]) {
                        if target_grid[i][j] != -1 && target_grid[i][j] != k as i32 {
                            macthed = false;
                            break;
                        }
                    }
                    if !macthed {
                        break;
                    }
                }
                if !macthed {
                    continue;
                }
                cnt -= 1;
                found = true;
                *ev = false;
                for i in (ri[0]..=ri[1]) {
                    for j in (ri[2]..=ri[3]) {
                        target_grid[i][j] = -1;
                    }
                }
            }
            if !found {
                break;
            }
        }
        cnt == 0
    }
}
// @lc code=end

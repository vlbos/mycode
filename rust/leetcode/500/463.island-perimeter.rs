/*
 * @lc app=leetcode id=463 lang=rust
 *
 * [463] Island Perimeter
 */

// @lc code=start
impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let mut s = 0;
        let mut a = Vec::new();
        for (i, r) in grid.iter().enumerate() {
            for (j, c) in r.iter().enumerate() {
                if *c == 1 {
                    a.push(i * 100 + j);
                    break;
                }
            }
            if !a.is_empty() {
                break;
            }
        }
        let mut ai = 0;
        let mut i = 0;
        let mut j = 0;
        while ai < a.len() {
            i=a[ai]/100;
            j=a[ai]%100;
            if i == 0 {
                s += 1;
            } else if grid[i - 1][j] == 0 {
                s += 1;
            } else {
                let n = (i - 1) * 100 + j;
                if !a.contains(&n) {
                    a.push(n);
                }
            }
            if i == grid.len() - 1 {
                s += 1;
            } else if grid[i + 1][j] == 0 {
                s += 1;
            } else {
                let n = (i + 1) * 100 + j;
                if !a.contains(&n) {
                    a.push(n);
                }
            }

            if j == 0 {
                s += 1;
            } else if grid[i][j - 1] == 0 {
                s += 1;
            } else {
                let n = i * 100 + j - 1;
                if !a.contains(&n) {
                    a.push(n);
                }
            }
            if j == grid[0].len() - 1 {
                s += 1;
            } else if grid[i][j + 1] == 0 {
                s += 1;
            } else {
                let n = i * 100 + j + 1;
                if !a.contains(&n) {
                    a.push(n);
                }
            }
            ai += 1;
        }
        // println!("{:?}",a);
        s
    }
}
// @lc code=end

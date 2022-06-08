/*
 * @lc app=leetcode id=378 lang=rust
 *
 * [378] Kth Smallest Element in a Sorted Matrix
 */

// @lc code=start
impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let check = |matrix: &Vec<Vec<i32>>, m: i32, k: i32| -> bool {
            let n = matrix.len();
            let mut i = n - 1;
            let mut j = 0;
            let mut num = 0;
            while i >= 0 && j < n {
                if matrix[i][j] <= m {
                    num += i + 1;
                    j += 1;
                } else {
                    if i == 0 {
                        break;
                    }
                    i -= 1;
                }
            }
            num>=k as usize
        };
        let mut l = matrix[0][0];
        let mut r = *(matrix.last().unwrap().last().unwrap());
        while l < r {
            let m = l+(r-l) / 2;
            if check(&matrix, m, k) {
                r = m;
            } else {
                l = m + 1;
            }
        }
        l
    }
}
// @lc code=end

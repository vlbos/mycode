/*
 * @lc app=leetcode id=1340 lang=rust
 *
 * [1340] Jump Game V
 */

// @lc code=start
impl Solution {
    pub fn max_jumps(arr: Vec<i32>, d: i32) -> i32 {
        let n = arr.len();
        let mut f = vec![-1; n];
        fn dfs(arr: &Vec<i32>, f: &mut Vec<i32>, id: usize, d: usize) {
            if f[id] != -1 {
                return;
            }
            f[id] = 1;
            for i in (0..id).rev() {
                if id - i > d || arr[id] <= arr[i] {
                    break;
                }
                dfs(arr, f, i, d);
                f[id] = f[id].max(f[i] + 1);
            }
            for i in id + 1..arr.len() {
                if i - id > d || arr[id] <= arr[i] {
                    break;
                }
                dfs(arr, f, i, d);
                f[id] = f[id].max(f[i] + 1);
            }
        }
        for i in 0..n {
            dfs(&arr, &mut f, i, d as usize);
        }
        f.into_iter().max().unwrap()
    }
}
// @lc code=end

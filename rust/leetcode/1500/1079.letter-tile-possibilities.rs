/*
 * @lc app=leetcode id=1079 lang=rust
 *
 * [1079] Letter Tile Possibilities
 */

// @lc code=start
impl Solution {
    pub fn num_tile_possibilities(tiles: String) -> i32 {
        let mut arr = vec![0; 26];
        for b in tiles.bytes() {
            arr[(b - b'A') as usize] += 1;
        }
        fn dfs(arr: &mut Vec<i32>) -> i32 {
            let mut sum = 0;
            for i in 0..26 {
                if arr[i] == 0 {
                    continue;
                }
                sum += 1;
                arr[i] -= 1;
                sum += dfs(arr);
                arr[i] += 1;
            }
            sum
        }
        dfs(&mut arr)
    }
}
// @lc code=end

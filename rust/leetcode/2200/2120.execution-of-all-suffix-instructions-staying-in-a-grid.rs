/*
 * @lc app=leetcode id=2120 lang=rust
 *
 * [2120] Execution of All Suffix Instructions Staying in a Grid
 */

// @lc code=start
impl Solution {
    pub fn execute_instructions(n: i32, start_pos: Vec<i32>, s: String) -> Vec<i32> {
        let bs = s.as_bytes();
        let m =  s.len();
        let mut ans = vec![0; m];
        for i in 0..m {
            let (mut x, mut y) = (start_pos[1], start_pos[0]);
            let mut cnt = m-i;
            for j in i..m {
                match bs[j as usize] {
                    b'L' => x-=1,
                    b'R' => x+=1,
                    b'U' => y-=1,
                    _ => y+=1,
                };
                if x < 0 || x >= n || y < 0 || y >= n {
                    cnt = j-i;    
                    break;
                }
            }
            ans[i] = cnt as i32;
        }
        ans
    }
}
// @lc code=end

/*
 * @lc app=leetcode id=1238 lang=rust
 *
 * [1238] Circular Permutation in Binary Representation
 */

// @lc code=start
impl Solution {
    pub fn circular_permutation(n: i32, start: i32) -> Vec<i32> {
       let n2 = 1<<n;
        let mut ans = vec![0;n2];
        for i in 0..n2{
            let ii= i as i32;
            ans[i ]=ii^(ii>>1)^start;
        }
        ans
    }
}
// @lc code=end


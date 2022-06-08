/*
 * @lc app=leetcode id=873 lang=rust
 *
 * [873] Length of Longest Fibonacci Subsequence
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
        let m = arr
            .iter()
            .enumerate()
            .map(|(i, v)| (*v as usize, i))
            .collect::<HashMap<usize, usize>>();
        let mut longest = HashMap::new();
        let mut ans = 0;
        let n = arr.len();
        for k in 0..n {
            for j in 0..k {
                let kj = (arr[k] - arr[j]) as usize;
                if arr[k] - arr[j] < arr[j] && m.contains_key(&kj) {
                    let i = m.get(&kj).unwrap_or(&0);
                    let ij = longest.get(&(i * n + j)).unwrap_or(&0) + 1;
                    longest.insert(n * j + k, ij);
                    ans = ans.max(longest.get(&(n * j + k)).unwrap_or(&0) + 2)
                }
            }
        }
        ans
    }
}
// @lc code=end

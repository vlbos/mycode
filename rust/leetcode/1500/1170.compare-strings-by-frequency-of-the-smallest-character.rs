/*
 * @lc app=leetcode id=1170 lang=rust
 *
 * [1170] Compare Strings by Frequency of the Smallest Character
 */

// @lc code=start
impl Solution {
    pub fn num_smaller_by_frequency(queries: Vec<String>, words: Vec<String>) -> Vec<i32> {
       let freq = |w: &String| -> i32 {
            let mut ans = vec![0; 26];
            for b in w.bytes() {
                ans[(b - b'a') as usize] += 1;
            }
            for &a in &ans {
                if a > 0 {
                    return a;
                }
            }
            0
        };
        let mut wf = Vec::new();
        for w in &words {
            wf.push(freq(w));
        }
        let mut ans = Vec::new();
        for q in &queries {
            let l = freq(q);
            ans.push(wf.iter().filter(|&x| *x > l).count() as i32);
        }
        ans
    }
}
// @lc code=end

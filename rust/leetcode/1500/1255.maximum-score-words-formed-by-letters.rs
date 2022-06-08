/*
 * @lc app=leetcode id=1255 lang=rust
 *
 * [1255] Maximum Score Words Formed by Letters
 */

// @lc code=start
impl Solution {
    pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut letter_cnt = HashMap::new();
        for &letter in &letters {
            *letter_cnt.entry(letter as u8 - b'a').or_insert(0) += 1;
        }
        let n = words.len();
        let group = |bit: i32| -> HashMap<u8, i32> {
            let mut g = HashMap::new();
            for i in 0..n {
                if bit & (1 << i) == 0 {
                    continue;
                }
                for b in words[i].bytes() {
                    *g.entry(b - b'a').or_insert(0) += 1;
                }
            }
            g
        };
        let calc_score = |g: &HashMap<u8, i32>| -> i32 {
            let mut ans = 0;
            for (j, &cnt) in g {
                if cnt > *letter_cnt.get(j).unwrap_or(&0) {
                    return 0;
                }
                ans += cnt * score[*j as usize];
            }
            ans
        };
        let mut ans = 0;
        for i in 0..(1 << n) {
            let g = group(i);
            ans = ans.max(calc_score(&g));
        }
        ans
    }
}
// @lc code=end

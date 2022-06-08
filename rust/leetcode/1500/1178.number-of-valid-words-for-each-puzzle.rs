/*
 * @lc app=leetcode id=1178 lang=rust
 *
 * [1178] Number of Valid Words for Each Puzzle
 */

// @lc code=start
impl Solution {
    pub fn find_num_of_valid_words(words: Vec<String>, puzzles: Vec<String>) -> Vec<i32> {
        let mut frequency = std::collections::HashMap::new();
        for word in &words {
            let mut mask:i32 = 0;
            for b in word.bytes() {
                mask |= 1 << (b - b'a');
            }
            if mask.count_ones() <= 7 {
                *frequency.entry(mask).or_insert(0) += 1;
            }
        }
        let mut ans = Vec::new();
        for puzzle in &puzzles {
            let mut total = 0;
            let mut mask = 0;
            for b in puzzle.bytes().skip(1) {
                mask |= 1 << (b - b'a');
            }
            let mut subset = mask;
            loop {
                let s = subset | (1 << (puzzle.as_bytes()[0] - b'a'));
                if let Some(&c) = frequency.get(&s) {
                    total += c;
                }
                subset = (subset - 1) & mask;
                if subset == mask {
                    break;
                }
            }
            ans.push(total);
        }
        ans
    }
}
// @lc code=end

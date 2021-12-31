/*
 * @lc app=leetcode id=948 lang=rust
 *
 * [948] Bag of Tokens
 */

// @lc code=start
impl Solution {
    pub fn bag_of_tokens_score(tokens: Vec<i32>, power: i32) -> i32 {
        if tokens.is_empty() {
            return 0;
        }
        let mut tokens = tokens;
        tokens.sort();
        let mut power = power;
        let (mut low, mut high) = (0, tokens.len() - 1);
        let mut points = 0;
        let mut ans = 0;
        while low <= high && (power >= tokens[low] || points > 0) {
            while low <= high && power >= tokens[low] {
                power -= tokens[low];
                points += 1;
                low += 1;
            }
            ans = ans.max(points);
            if low <= high && points > 0 {
                points -= 1;
                power += tokens[high];
                high -= 1;
            }
        }
        ans
    }
}
// @lc code=end

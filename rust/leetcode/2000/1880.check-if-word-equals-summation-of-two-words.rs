/*
 * @lc app=leetcode id=1880 lang=rust
 *
 * [1880] Check if Word Equals Summation of Two Words
 */

// @lc code=start
impl Solution {
    pub fn is_sum_equal(first_word: String, second_word: String, target_word: String) -> bool {
        let sum = |x:String|->i32{
            let mut ans = 0;
            for c in x.chars(){
                ans*=10;
                ans+= (c as u8-'a' as u8) as i32;
            }
            ans
        };
        sum(first_word)+sum(second_word)==sum(target_word)
    }
}
// @lc code=end


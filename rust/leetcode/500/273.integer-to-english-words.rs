/*
 * @lc app=leetcode id=273 lang=rust
 *
 * [273] Integer to English Words
 */

// @lc code=start
impl Solution {
    pub fn number_to_words(num: i32) -> String {
        let singles = [
            "", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine",
        ];
        let teens = [
            "Ten",
            "Eleven",
            "Twelve",
            "Thirteen",
            "Fourteen",
            "Fifteen",
            "Sixteen",
            "Seventeen",
            "Eighteen",
            "Nineteen",
        ];
        let tens = [
            "", "Ten", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety",
        ];
        let thousands = ["", "Thousand", "Million", "Billion"];
        if num == 0 {
            return "Zero".to_string();
        }
        let to_english = |mut num: i32| -> String {
            let mut ans = String::new();
            let hundred = num / 100;
            num %= 100;
            if hundred > 0 {
                ans.push_str((singles[hundred as usize].to_string() + " Hundred ").as_str());
            }
            let ten = num / 10;

            if ten >= 2 {
                ans.push_str((tens[ten as usize].to_string() + " ").as_str());
                num %= 10;
            }
            if num > 0 && num < 10 {
                ans.push_str((singles[num as usize].to_string() + " ").as_str());
            } else if num >= 10 {
                ans.push_str((teens[num as usize - 10].to_string() + " ").as_str());
            }
            ans
        };
        let mut unit = 1000_000_000;
        let mut ans = String::new();
        let mut num = num;
        for i in (0..=3).rev() {
            let cur = num / unit;
            if cur > 0 {
                num -= cur * unit;
                ans.push_str((to_english(cur) + thousands[i] + " ").as_str());
            }
            unit /= 1000;
        }
        ans.trim().to_string()
    }
}
// @lc code=end

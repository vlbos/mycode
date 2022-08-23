// 1618\. Maximum Font to Fit a Sentence in a Screen
// =================================================

// You are given a string `text`. We want to display `text` on a screen of width `w` and height `h`.
//  You can choose any font size from array `fonts`, which contains the available font sizes **in ascending order**.

// You can use the `FontInfo` interface to get the width and height of any character at any available font size.

// The `FontInfo` interface is defined as such:

// interface FontInfo {
//   // Returns the width of character ch on the screen using font size fontSize.
//   // O(1) per call
//   public int getWidth(int fontSize, char ch);

//   // Returns the height of any character on the screen using font size fontSize.
//   // O(1) per call
//   public int getHeight(int fontSize);
// }

// The calculated width of `text` for some `fontSize` is the **sum** of every `getWidth(fontSize, text[i])` call for each `0 <= i < text.length` (**0-indexed**).
// The calculated height of `text` for some `fontSize` is `getHeight(fontSize)`. Note that `text` is displayed on a **single line**.

// It is guaranteed that `FontInfo` will return the same value if you call `getHeight` or `getWidth` with the same parameters.

// It is also guaranteed that for any font size `fontSize` and any character `ch`:

// *   `getHeight(fontSize) <= getHeight(fontSize+1)`
// *   `getWidth(fontSize, ch) <= getWidth(fontSize+1, ch)`

// Return _the maximum font size you can use to display_ `text` _on the screen_. If `text` cannot fit on the display with any font size, return `-1`.

// **Example 1:**

// **Input:** text = "helloworld", w = 80, h = 20, fonts = \[6,8,10,12,14,16,18,24,36\]
// **Output:** 6

// **Example 2:**

// **Input:** text = "leetcode", w = 1000, h = 50, fonts = \[1,2,4\]
// **Output:** 4

// **Example 3:**

// **Input:** text = "easyquestion", w = 100, h = 100, fonts = \[10,15,20,25\]
// **Output:** -1

// **Constraints:**

// *   `1 <= text.length <= 50000`
// *   `text` contains only lowercase English letters.
// *   `1 <= w <= 107`
// *   `1 <= h <= 104`
// *   `1 <= fonts.length <= 105`
// *   `1 <= fonts[i] <= 105`
// *   `fonts` is sorted in ascending order and does not contain duplicates.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Google](https://leetcode.ca/tags/#Google)

// maximum_font_to_fit_a_sentence_in_a_screen

// int max_font(String text, int w, int h, int[] fonts, FontInfo fontInfo)

pub struct FontInfo;
impl FontInfo {
    pub fn get_height(&self, font_size: i32) -> i32 {
        font_size
    }
    pub fn get_width(&self, font_size: i32, ch: char) -> i32 {
        (((ch as u8 as f32 * 1.5) as u8 / (ch as u8)) as i32) * font_size
    }
}
#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn max_font(text: String, w: i32, h: i32, fonts: Vec<i32>, font_info: FontInfo) -> i32 {
        let can_fit = |text: &String, w: i32, h: i32, font: i32, font_info: &FontInfo| {
            let sum_width = text
                .chars()
                .map(|c| font_info.get_width(font, c))
                .sum::<i32>();
            println!("{},{},{},{}", font_info.get_height(font), h, w, sum_width);
            font_info.get_height(font) <= h && sum_width <= w
        };
        let (mut low, mut high) = (0, fonts.len() as i32 - 1);
        while low <= high {
            let mid = low + (high - low) / 2;
            if can_fit(&text, w, h, fonts[mid as usize], &font_info) {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }
        if low > 0 {
            fonts[low as usize - 1]
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_max_font_1() {
        assert_eq!(
            8,
            Solution::max_font(
                String::from("helloworld"),
                80,
                20,
                vec![6, 8, 10, 12, 14, 16, 18, 24, 36],
                FontInfo {}
            )
        );
    }
    #[test]
    pub fn test_max_font_2() {
        assert_eq!(
            4,
            Solution::max_font(
                String::from("leetcode"),
                1000,
                50,
                vec![1, 2, 4],
                FontInfo {}
            )
        );
    }
    #[test]
    pub fn test_max_font_3() {
        assert_eq!(
            -1,
            Solution::max_font(
                String::from("easyquestion"),
                100,
                100,
                vec![10, 15, 20, 25],
                FontInfo {}
            )
        );
    }
}

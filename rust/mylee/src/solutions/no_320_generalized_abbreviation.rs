// 320\. Generalized Abbreviation
// ==============================

// Write a function to generate the generalized abbreviations of a word.

// **Note:** The order of the output does not matter.

// **Example:**

// **Input:** `"word"`
// **Output:**
// \["word", "1ord", "w1rd", "wo1d", "wor1", "2rd", "w2d", "wo2", "1o1d", "1or1", "w1r1", "1o2", "2r1", "3d", "w3", "4"\]

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Google](https://leetcode.ca/tags/#Google)

// @lc code=start
impl Solution {
    pub fn generate_abbreviations(word: String) -> Vec<String> {
        // let chars = word.chars().collect::<Vec<char>>();
        // let len = chars.len();
        // let size = 1 << len;
        // let factors = (0..len).map(|f| 1 << f).collect::<Vec<usize>>();
        // (0..size)
        //     .map(|i| {
        //         let mut word = chars.clone();
        //         for j in 0..len {
        //             if i & factors[j] == 0 {
        //                 word[j] = '#';
        //             }
        //         }
        //         let mut new_word = String::new();
        //         let mut count = 0;
        //         for j in 0..len {
        //             if word[j] == '#' {
        //                 count += 1;
        //             } else {
        //                 if count != 0 {
        //                     new_word += &count.to_string();
        //                 }
        //                 new_word.push(word[j]);
        //                 count = 0;
        //             }
        //         }
        //         if count != 0 {
        //             new_word += &count.to_string();
        //         }
        //         new_word
        //     })
        //     .collect()
        let n = word.len();
        let n1 = 1 << n;
        (0..n1)
            .map(|i| {
                let s = format!("{:01$b}", i, n);
                let mut ans = String::new();
                let mut j = 0;
                let bs = s.as_bytes();
                while j < bs.len() {
                    if bs[j] == b'0' {
                        ans.push(word.as_bytes()[j] as char);
                        j += 1;
                        continue;
                    }
                    let k = j;
                    while j < bs.len() && bs[j] == b'1' {
                        j += 1;
                    }
                    ans.push_str(&word[k..j].len().to_string().as_str());
                }
                ans
            })
            .collect()
    }
}
// @lc code=end

#[allow(dead_code)]
pub  struct Solution;

#[cfg(test)]
mod test {

    use super::*;
    use crate::solutions::util::test_tools::assert_equivalent;

    #[test]
    fn test_generate_abbreviations() {
        let word = String::from("word");
        assert_equivalent(
            &Solution::generate_abbreviations(word),
            &(vec![
                "word", "1ord", "w1rd", "wo1d", "wor1", "2rd", "w2d", "wo2", "1o1d", "1or1",
                "w1r1", "1o2", "2r1", "3d", "w3", "4",
            ])
            .into_iter()
            .map(String::from)
            .collect::<Vec<String>>(),
        );
    }
}

// 411\. Minimum Unique Word Abbreviation
// ======================================

// A string such as `"word"` contains the following abbreviations:

// \["word", "1ord", "w1rd", "wo1d", "wor1", "2rd", "w2d", "wo2", "1o1d", "1or1", "w1r1", "1o2", "2r1", "3d", "w3", "4"\]

// Given a target string and a set of strings in a dictionary, find an abbreviation of this target string with the **_smallest possible_** length
// such that it does not conflict with abbreviations of the strings in the dictionary.

// Each **number** or letter in the abbreviation is considered length = 1. For example, the abbreviation "a32bc" has length = 4.

// **Note:**

// *   In the case of multiple answers as shown in the second example below, you may return any one of them.
// *   Assume length of target string = **m**, and dictionary size = **n**. You may assume that **m ≤ 21**, **n ≤ 1000**, and **log2(n) + m ≤ 20**.

// **Examples:**

// "apple", \["blade"\] -> "a4" (because "5" or "4e" conflicts with "blade")

// "apple", \["plain", "amber", "blade"\] -> "1p3" (other valid answers include "ap3", "a3e", "2p2", "3le", "3l1").

// ### Difficulty:

// Hard

// ### Lock:

// Prime

// ### Company:

// [Google](https://leetcode.ca/tags/#Google)

// @lc code=start
impl Solution {
    pub fn min_abbreviation(target: String, dictionary: Vec<String>) -> String {
        // let chars = target.chars().collect::<Vec<char>>();
        // let len = chars.len();
        // if len <= 0 {
        //     return target;
        // }
        // let size = 1usize << len;
        // let dic_same = dictionary
        //     .into_iter()
        //     .map(|s| s.chars().collect::<Vec<char>>())
        //     .filter(|v| v.len() == len)
        //     .map(|d| {
        //         let mut same = 0;
        //         for i in 0..len {
        //             same += if d[i] != chars[i] { 1usize << i } else { 0 }
        //         }
        //         same
        //     })
        //     .collect::<Vec<usize>>();
        // if dic_same.len() == 0 {
        //     return len.to_string();
        // }

        // let factors = (0..len).map(|i| 1 << i).collect::<Vec<usize>>();

        // let mut min_len = usize::max_value();
        // let mut abbr = 0;
        // 'outer: for i in 0..size {
        //     for d in &dic_same {
        //         if d & i == 0 {
        //             continue 'outer;
        //         }
        //     }
        //     let abbr_len = Solution::abbr_to_len(i, &factors);
        //     if abbr_len < min_len {
        //         min_len = abbr_len;
        //         abbr = i;
        //     }
        // }
        // match abbr {
        //     0 => target,
        //     i => Solution::abbr_to_string(i, &chars, &factors),
        // }
        let n = target.len();
        let bn = 1 << n;
        let mut candidates = 0;
        let mut min_len = i32::MAX;
        let mut dict = Vec::new();
        for w in &dictionary {
            if w.len() != n {
                continue;
            }
            let mut word = 0;
            let bw = w.as_bytes();
            for (i, b) in target.bytes().enumerate() {
                word <<= 1;
                word += if b != bw[i] { 1 } else { 0 };
            }
            dict.push(word);
            candidates |= word;
        }
        let mut min_ab = 0;
        fn dfs(
            dict: &Vec<i32>,
            bit: i32,
            mask: i32,
            min_len: &mut i32,
            min_ab: &mut i32,
            candidates: i32,
            bn: i32,
            n: usize,
        ) {
            let abbr_len = || {
                let mut cnt = n as i32;
                let mut b = 3;
                while b < bn {
                    if (mask & b )== 0 {
                        cnt -= 1;
                    }
                    b <<= 1;
                }

                cnt
            };
            let len = abbr_len();
            if len >= *min_len {
                return;
            }
            let mut matches = true;
            for &d in dict {
                if (mask & d) == 0 {
                    matches = false;
                    break;
                }
            }
            if matches {
                *min_len = len;
                *min_ab = mask;
            } else {
                let mut b = bit;
                while b < bn {
                    if (candidates & b) > 0 {
                        dfs(dict, b << 1, mask + b, min_len, min_ab, candidates, bn, n);
                    }
                    b <<= 1;
                }
            }
        }
        dfs(&dict, 1, 0, &mut min_len, &mut min_ab, candidates, bn, n);
        let mut ans = String::new();
        let mut pre = n - 1;
        for i in (0..n).rev() {
            if  (min_ab & 1) > 0 {
                if pre > i  {
                    ans = (pre - i).to_string() + ans.as_str();
                }
                if i>0{
                pre = i - 1;
                }
                ans = target[i..i + 1].to_string() + ans.as_str();
            } else if i == 0 {
                ans = (pre + 1 - i).to_string() + ans.as_str();
            }
            min_ab >>= 1;
        }
        ans
    }

    //
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_min_abbreviation_1() {
        assert_eq!(
            Solution::min_abbreviation(String::from("apple"), vec![String::from("blade")]),
            String::from("a4")
        );
    }

    #[test]
    fn test_min_abbreviation_2() {
        let set = vec!["ap3", "a3e", "2p2", "3le", "3l1", "1p3"]
            .into_iter()
            .map(String::from)
            .collect::<HashSet<String>>();
        assert!(set.contains(&Solution::min_abbreviation(
            String::from("apple"),
            vec![
                String::from("plain"),
                String::from("amber"),
                String::from("blade")
            ]
        )));
    }
}

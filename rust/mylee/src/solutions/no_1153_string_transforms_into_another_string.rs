// 1153\. String Transforms Into Another String
// ============================================

// Given two strings `str1` and `str2` of the same length, determine whether you can transform `str1` into `str2` by doing **zero or more** _conversions_.

// In one conversion you can convert **all** occurrences of one character in `str1` to **any** other lowercase English character.

// Return `true` if and only if you can transform `str1` into `str2`.

// **Example 1:**

// **Input:** str1 = "aabcc", str2 = "ccdee"
// **Output:** true
// **Explanation:** Convert 'c' to 'e' then 'b' to 'd' then 'a' to 'c'. Note that the order of conversions matter.

// **Example 2:**

// **Input:** str1 = "leetcode", str2 = "codeleet"
// **Output:** false
// **Explanation:** There is no way to transform str1 to str2.

// **Note:**

// 1.  `1 <= str1.length == str2.length <= 10^4`
// 2.  Both `str1` and `str2` contain only lowercase English letters.

// ### Difficulty:

// Hard

// ### Lock:

// Prime

// ### Company:

// [Google](https://leetcode.ca/tags/#Google)

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn can_convert(str1: String, str2: String) -> bool {
        if str1 == str2 {
            return true;
        }
        let mut m = std::collections::HashMap::new();
        for (c1, c2) in str1.chars().zip(str2.chars()) {
            if let Some(&c) = m.get(&c1) {
                if c != c2 {
                    return false;
                }
            }
            m.insert(c1, c2);
        }
        m.values()
            .cloned()
            .collect::<std::collections::HashSet<char>>()
            .len()
            < 26
    }
}

// impl Solution {
//     pub fn most_visited_pattern(username: Vec<String>, timestamp: Vec<i32>, website: Vec<String>) -> Vec<String> {
// use std::collections::{HashMap,HashSet};
//        let n = username.len();
//         let mut mp = HashMap::new();
//         let mut seq = HashMap::new();

//         for i in (0..n) {
//             mp.entry(&username[i]).or_insert(vec![]).push((timestamp[i], &website[i]));
//         }

//         for (u, v) in mp.iter_mut() {
//             v.sort_by(|a, b| a.0.cmp(&b.0));
//             let n = v.len();

//             for i in 0..n {
//                 for j in i+1..n {
//                     for k in j+1..n {
//                         seq.entry((v[i].1, v[j].1, v[k].1)).or_insert(HashSet::new()).insert(u);
//                     }
//                 }
//             }
//         }

//         let empty = "".to_string();
//         let mut max = 0;
//         let mut ans = (&empty, &empty, &empty);

//         for (k, v) in seq {
//             if v.len() > max || (v.len() == max && k < ans){
//                 max = v.len();
//                 ans = k;
//             }
//         }

//         vec![ans.0.to_string(), ans.1.to_string(), ans.2.to_string()]
//     }
// }
#[cfg(test)]
mod test {
    use super::*;

    // ["zkiikgv","zkiikgv","zkiikgv","zkiikgv"]
    // [436363475,710406388,386655081,797150921]
    // ["wnaaxbfhxp","mryxsjc","oz","wlarkzzqht"]
    // 输出：
    // ["oz","wnaaxbfhxp","mryxsjc"]
    // 预期结果：
    // ["oz","mryxsjc","wlarkzzqht"]
    #[test]
    pub fn test_can_convert_1() {
        assert!(Solution::can_convert(
            String::from("aabcc"),
            String::from("ccdee")
        ));
    }
    #[test]
    pub fn test_can_convert_2() {
        assert!(!Solution::can_convert(
            String::from("leetcode"),
            String::from("codeleet")
        ));
    }
}

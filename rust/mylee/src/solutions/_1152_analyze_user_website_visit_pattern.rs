// 1152\. Analyze User Website Visit Pattern
// =========================================

// We are given some website visits: the user with name `username[i]` visited the website `website[i]` at time `timestamp[i]`.

// A _3-sequence_ is a list of websites of length 3 sorted in ascending order by the time of their visits.  (The websites in a 3-sequence are not necessarily distinct.)

// Find the 3-sequence visited by the largest number of users. If there is more than one solution, return the lexicographically smallest such 3-sequence.

// **Example 1:**

// **Input:** username = \["joe","joe","joe","james","james","james","james","mary","mary","mary"\], timestamp = \[1,2,3,4,5,6,7,8,9,10\], website = \["home","about","career","home","cart","maps","home","home","about","career"\]
// **Output:** \["home","about","career"\]
// **Explanation:**
// The tuples in this example are:
// \["joe", 1, "home"\]
// \["joe", 2, "about"\]
// \["joe", 3, "career"\]
// \["james", 4, "home"\]
// \["james", 5, "cart"\]
// \["james", 6, "maps"\]
// \["james", 7, "home"\]
// \["mary", 8, "home"\]
// \["mary", 9, "about"\]
// \["mary", 10, "career"\]
// The 3-sequence ("home", "about", "career") was visited at least once by **2** users.
// The 3-sequence ("home", "cart", "maps") was visited at least once by 1 user.
// The 3-sequence ("home", "cart", "home") was visited at least once by 1 user.
// The 3-sequence ("home", "maps", "home") was visited at least once by 1 user.
// The 3-sequence ("cart", "maps", "home") was visited at least once by 1 user.

// **Note:**

// 1.  `3 <= N = username.length = timestamp.length = website.length <= 50`
// 2.  `1 <= username[i].length <= 10`
// 3.  `0 <= timestamp[i] <= 10^9`
// 4.  `1 <= website[i].length <= 10`
// 5.  Both `username[i]` and `website[i]` contain only lowercase characters.
// 6.  It is guaranteed that there is at least one user who visited at least 3 websites.
// 7.  No user visits two websites at the same time.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon)

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn most_visited_pattern(
        username: Vec<String>,
        timestamp: Vec<i32>,
        website: Vec<String>,
    ) -> Vec<String> {
        use std::collections::HashMap;
        let mut uw = HashMap::new();
        let mut cnt = HashMap::new();
        let mut timestamp: Vec<(i32, usize)> = timestamp
            .into_iter()
            .enumerate()
            .map(|(i, v)| (v, i))
            .collect();
        timestamp.sort();
        for &(_, i) in &timestamp {
            uw.entry(username[i].clone())
                .or_insert(Vec::new())
                .push(website[i].clone());
        }
        for v in uw.values() {
            for w in v.windows(3) {
                *cnt.entry(w.to_vec()).or_insert(0) += 1;
            }
        }
        let mut cnt: Vec<(i32, Vec<String>)> = cnt.iter().map(|(k, &v)| (-v, k.clone())).collect();
        cnt.sort();
        cnt[0].1.clone()
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

    #[test]
    pub fn test_most_visited_pattern_1() {
        assert_eq!(
            ["home", "about", "career"]
                .into_iter()
                .map(String::from)
                .collect::<Vec<String>>(),
            Solution::most_visited_pattern(
                ["joe", "joe", "joe", "james", "james", "james", "james", "mary", "mary", "mary"]
                    .into_iter()
                    .map(String::from)
                    .collect::<Vec<String>>(),
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
                [
                    "home", "about", "career", "home", "cart", "maps", "home", "home", "about",
                    "career"
                ]
                .into_iter()
                .map(String::from)
                .collect::<Vec<String>>()
            )
        );
    }
}

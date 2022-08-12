// 267\. Palindrome Permutation II
// ===============================

// Given a string `s`, return all the palindromic permutations (without duplicates) of it.
// Return an empty list if no palindromic permutation could be form.

// **Example 1:**

// **Input:** `"aabb"`
// **Output:** `["abba", "baab"]`

// **Example 2:**

// **Input:** `"abc"`
// **Output:** `[]`

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [Google](https://leetcode.ca/tags/#Google) [Uber](https://leetcode.ca/tags/#Uber)

// @lc code=start
// use std::cell::RefCell;
// use std::rc::Rc;

// use std::collections::HashMap;

// type CharCounter = HashMap<char, usize>;

// pub fn   count(counter: &mut CharCounter, ch: char) {
//     counter.entry(ch).and_modify(|c| *c += 1).or_insert(1);
// }

// pub fn   decount(counter: &mut CharCounter, ch: char, times: usize) {
//     let count = *counter.get(&ch).unwrap();
//     if times >= count {
//         counter.remove(&ch);
//     } else {
//         counter.insert(ch, count - times);
//     }
// }

impl Solution {
    pub fn   generate_palindromes(s: String) -> Vec<String> {
        // let mut counter = CharCounter::new();
        // for c in s.chars() {
        //     count(&mut counter, c);
        // }
        // let mut odd_center: Option<char> = None;
        // if (s.len() & 1) == 1 {
        //     for (k, v) in &counter {
        //         if (v & 1) == 1 {
        //             match odd_center {
        //                 None => odd_center = Some(*k),
        //                 _ => return vec![],
        //             }
        //         }
        //     }
        // } else {
        //     for (_, v) in &counter {
        //         if (v & 1) == 1 {
        //             return vec![];
        //         }
        //     }
        // }
        // if let Some(odd_char) = &odd_center {
        //     decount(&mut counter, *odd_char, 1);
        // }
        // let res_rc = Rc::new(RefCell::new(vec![]));
        // Solution::generate_palindromes_rec(res_rc.clone(), counter, String::from(""));
        // let src = res_rc.borrow();
        // let mut res = vec![];
        // if let Some(odd_char) = &odd_center {
        //     for r in src.iter() {
        //         let mut new_r = r.clone();
        //         new_r.push(*odd_char);
        //         new_r.extend(r.chars().rev());
        //         res.push(new_r);
        //     }
        // } else {
        //     for r in src.iter() {
        //         let mut new_r = r.clone();
        //         new_r.extend(r.chars().rev());
        //         res.push(new_r);
        //     }
        // }
        // res
        let mut cnt = std::collections::HashMap::new();
        for c in s.chars() {
            *cnt.entry(c).or_insert(0) += 1;
        }
        if cnt.values().filter(|x| *x % 2 == 1).count() > 1 {
            return Vec::new();
        }
        let target: Vec<char> = cnt
            .iter()
            .filter(|x| *x.1 % 2 == 0)
            .map(|x| vec![*x.0; *x.1 / 2])
            .flatten()
            .collect();
        let mut pre = std::collections::HashSet::from([target.clone()]);
        let n = target.len();
        for i in 0..n - 1 {
            for j in i + 1..n {
                if target[i] == target[j] {
                    continue;
                }
                let mut t = target.clone();
                t.swap(i, j);
                pre.insert(t);
            }
        }
        let mid = if let Some(x) = cnt.iter().filter(|x| *x.1 % 2 == 1).next() {
            x.0.to_string()
        } else {
            String::new()
        };
        let mut ans = Vec::new();
        for s in &pre {
            let mut ss = s.clone();
            ss.reverse();
            ans.push(format!(
                "{}{}{}",
                s.iter().collect::<String>(),
                mid,
                ss.iter().collect::<String>()
            ));
        }
        ans
    }

    //pub fn  generate_palindromes_rec(
    //     res: Rc<RefCell<Vec<String>>>,
    //     counter: CharCounter,
    //     visited: String,
    // ) {
    //     if counter.len() == 0 {
    //         let mut res_mb = res.borrow_mut();
    //         res_mb.push(visited);
    //     } else {
    //         for k in counter.keys() {
    //             let mut new_visited = visited.clone();
    //             new_visited.push(*k);
    //             let mut new_counter = counter.clone();
    //             decount(&mut new_counter, *k, 2);
    //             Solution::generate_palindromes_rec(res.clone(), new_counter, new_visited);
    //         }
    //     }
    // }
}
// @lc code=end

#[allow(dead_code)]
pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    use crate::solutions::util::test_tools::{assert_equivalent, map_to_string};

    #[test]
   pub fn  test_generate_palindromes() {
        let target = map_to_string(&["abba", "baab"]);
        assert_equivalent(
            &Solution::generate_palindromes(String::from("aabb")),
            &target,
        );
        assert_equivalent(
            &Solution::generate_palindromes(String::from("aba")),
            &vec![String::from("aba")],
        );
    }
}

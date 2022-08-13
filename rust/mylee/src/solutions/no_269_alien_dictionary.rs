// 269\. Alien Dictionary
// ======================

// There is a new alien language which uses the latin alphabet.
// However, the order among letters are unknown to you.
// You receive a list of **non-empty** words from the dictionary,
// where **words are sorted lexicographically by the rules of this new language**.
// Derive the order of letters in this language.

// **Example 1:**

// **Input:**
// \[
//   "wrt",
//   "wrf",
//   "er",
//   "ett",
//   "rftt"
// \]

// **Output:** `"wertf"`

// **Example 2:**

// **Input:**
// \[
//   "z",
//   "x"
// \]

// **Output:** `"zx"`

// **Example 3:**

// **Input:**
// \[
//   "z",
//   "x",
//   "z"
// \]

// **Output:** `""`

// **Explanation:** The order is invalid, so return `""`.

// **Note:**

// 1.  You may assume all letters are in lowercase.
// 2.  You may assume that if a is a prefix of b, then a must appear before b in the given dictionary.
// 3.  If the order is invalid, return an empty string.
// 4.  There may be multiple valid order of letters, return any one of them is fine.

// ### Difficulty:

// Hard

// ### Lock:

// Prime

// ### Company:

// [Airbnb](https://leetcode.ca/tags/#Airbnb) [Amazon](https://leetcode.ca/tags/#Amazon) [Apple](https://leetcode.ca/tags/#Apple) [Bloomberg](https://leetcode.ca/tags/#Bloomberg) [Cohesity](https://leetcode.ca/tags/#Cohesity) [Facebook](https://leetcode.ca/tags/#Facebook) [Flipkart](https://leetcode.ca/tags/#Flipkart) [Google](https://leetcode.ca/tags/#Google) [Microsoft](https://leetcode.ca/tags/#Microsoft) [Oracle](https://leetcode.ca/tags/#Oracle) [Pinterest](https://leetcode.ca/tags/#Pinterest) [Pocket Gems](https://leetcode.ca/tags/#Pocket%20Gems) [Snapchat](https://leetcode.ca/tags/#Snapchat) [Square](https://leetcode.ca/tags/#Square) [Twitter](https://leetcode.ca/tags/#Twitter) [Uber](https://leetcode.ca/tags/#Uber) [VMware](https://leetcode.ca/tags/#VMware)
// @lc code=start
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::iter::FromIterator;

// #[derive(Clone, Debug)]
// pub  struct  GraphNode {
//     ins: HashSet<char>,
//     outs: HashSet<char>,
// }

// impl GraphNode {
//    pub fn  new() -> GraphNode {
//         GraphNode {
//             ins: HashSet::new(),
//             outs: HashSet::new(),
//         }
//     }
// }

impl Solution {
    //pub fn  link_graph(
    //     words: &Vec<Vec<char>>,
    //     c: usize,
    //     rs: usize,
    //     re: usize,
    //     nodes: &mut HashMap<char, GraphNode>,
    // ) -> bool {
    //     if rs + 1 >= re {
    //         return true;
    //     }
    //     let mut orders: Vec<(char, usize, usize)> = vec![];
    //     let mut last_empty = rs;
    //     for i in rs..re {
    //         if words[i].len() > c {
    //             let ch = words[i][c];
    //             if orders.is_empty() || orders[orders.len() - 1].0 != ch {
    //                 orders.push((ch, i, i + 1));
    //             } else {
    //                 let end = orders.len() - 1;
    //                 orders[end].2 = i + 1;
    //             }
    //         } else {
    //             if i != last_empty {
    //                 return false;
    //             } else {
    //                 last_empty += 1;
    //             }
    //         }
    //     }
    //     for i in 1..orders.len() {
    //         let prev = &orders[i - 1].0;
    //         let curr = &orders[i].0;
    //         nodes.get_mut(curr).unwrap().ins.insert(*prev);
    //         nodes.get_mut(prev).unwrap().outs.insert(*curr);
    //     }
    //     for o in orders {
    //         if !Solution::link_graph(words, c + 1, o.1, o.2, nodes) {
    //             return false;
    //         }
    //     }
    //     true
    // }

    pub fn alien_order(words: Vec<String>) -> String {
        // let words = words
        //     .iter()
        //     .map(|s| s.chars().collect::<Vec<char>>())
        //     .collect::<Vec<Vec<char>>>();
        // let mut nodes: HashMap<char, GraphNode> = HashMap::from_iter(
        //     HashSet::<&char>::from_iter(words.iter().flatten())
        //         .into_iter()
        //         .map(|c| (*c, GraphNode::new())),
        // );
        // if !Solution::link_graph(&words, 0, 0, words.len(), &mut nodes) {
        //     return String::new();
        // }
        // let mut res = String::new();
        // loop {
        //     let mut changed = false;
        //     let zero_in_nodes = nodes
        //         .iter()
        //         .filter(|(_, n)| n.ins.is_empty())
        //         .map(|(k, _)| *k)
        //         .collect::<Vec<char>>();
        //     for n in zero_in_nodes {
        //         let outs = nodes.get(&n).unwrap().outs.clone();
        //         for o in outs {
        //             nodes.get_mut(&o).unwrap().ins.remove(&n);
        //         }
        //         nodes.remove(&n);
        //         res.push(n);
        //         changed = true;
        //     }
        //     if !changed {
        //         break;
        //     }
        // }
        // if !nodes.is_empty() {
        //     String::new()
        // } else {
        //     res
        // }
        if words.is_empty() {
            return String::new();
        }
        if words.len() == 1 {
            return words[0].clone();
        }
        use std::collections::HashMap;
        use std::collections::HashSet;
        let mut degree: HashMap<u8, i32> = words
            .iter()
            .map(|s| s.bytes().collect::<Vec<u8>>())
            .flatten()
            .map(|x| (x, 0))
            .collect();
        let mut m = HashMap::new();
        for w in words.windows(2) {
            let len = w[0].len().min(w[1].len());
            let (w1, w2) = (w[0].as_bytes(), w[1].as_bytes());
            for j in 0..len {
                let (c1, c2) = (w1[j], w2[j]);

                if c1 != c2 {
                    if !m.entry(c1).or_insert(HashSet::new()).contains(&c2) {
                        m.entry(c1).or_insert(HashSet::new()).insert(c2);
                        *degree.entry(c2).or_insert(0) += 1;
                    }
                    break;
                }
            }
        }
        let mut q = std::collections::VecDeque::from(
            degree
                .iter()
                .filter(|x| *x.1 == 0)
                .map(|x| *x.0)
                .collect::<Vec<u8>>(),
        );
        let mut ans = String::new();
        while let Some(c) = q.pop_front() {
            ans.push(c as char);
            for &c2 in m.get(&c).unwrap_or(&HashSet::new()) {
                *degree.entry(c2).or_insert(0) -= 1;
                if *degree.get(&c2).unwrap_or(&0) == 0 {
                    q.push_back(c2);
                }
            }
        }
        if ans.len() != degree.len() {
            return String::new();
        }
        ans
    }
}

// @lc code=end

#[allow(dead_code)]
pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    use crate::solutions::util::test_tools::map_to_string;

    #[test]
    pub fn test_alien_order1() {
        let inputs = map_to_string(&["wrt", "wrf", "er", "ett", "rftt"]);
        assert_eq!(Solution::alien_order(inputs), String::from("wertf"));
    }

    #[test]
    pub fn test_alien_order2() {
        let inputs = map_to_string(&["z", "x"]);
        assert_eq!(Solution::alien_order(inputs), String::from("zx"));
    }

    #[test]
    pub fn test_alien_order3() {
        let inputs = map_to_string(&["z", "x", "z"]);
        assert_eq!(Solution::alien_order(inputs), String::from(""));
    }
}

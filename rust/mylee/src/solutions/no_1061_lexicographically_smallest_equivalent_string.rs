// 1061\. Lexicographically Smallest Equivalent String
// ===================================================

// Given strings `A` and `B` of the same length, we say A\[i\] and B\[i\] are equivalent characters.
// For example, if `A = "abc"` and `B = "cde"`, then we have `'a' == 'c', 'b' == 'd', 'c' == 'e'`.

// Equivalent characters follow the usual rules of any equivalence relation:

// *   Reflexivity: 'a' == 'a'
// *   Symmetry: 'a' == 'b' implies 'b' == 'a'
// *   Transitivity: 'a' == 'b' and 'b' == 'c' implies 'a' == 'c'

// For example, given the equivalency information from `A` and `B` above, `S = "eed"`, `"acd"`, and `"aab"` are equivalent strings,
// and `"aab"` is the lexicographically smallest equivalent string of `S`.

// Return the lexicographically smallest equivalent string of `S` by using the equivalency information from `A` and `B`.

// **Example 1:**

// **Input:** A = "parker", B = "morris", S = "parser"
// **Output:** "makkek"
// **Explanation:** Based on the equivalency information in `A` and `B`, we can group their characters as `[m,p]`, `[a,o]`, `[k,r,s]`, `[e,i]`.
// The characters in each group are equivalent and sorted in lexicographical order. So the answer is `"makkek"`.

// **Example 2:**

// **Input:** A = "hello", B = "world", S = "hold"
// **Output:** "hdld"
// **Explanation: ** Based on the equivalency information in `A` and `B`, we can group their characters as `[h,w]`, `[d,e,o]`, `[l,r]`.
// So only the second letter `'o'` in `S` is changed to `'d'`, the answer is `"hdld"`.

// **Example 3:**

// **Input:** A = "leetcode", B = "programs", S = "sourcecode"
// **Output:** "aauaaaaada"
// **Explanation: ** We group the equivalent characters in `A` and `B` as `[a,o,e,r,s,c]`, `[l,p]`, `[g,t]` and `[d,m]`,
// thus all letters in `S` except `'u'` and `'d'` are transformed to `'a'`, the answer is `"aauaaaaada"`.

// **Note:**

// 1.  String `A`, `B` and `S` consist of only lowercase English letters from `'a'` \- `'z'`.
// 2.  The lengths of string `A`, `B` and `S` are between `1` and `1000`.
// 3.  String `A` and `B` are of the same length.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Unknown](https://leetcode.ca/tags/#Unknown)

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
        use std::collections::{HashMap, HashSet};
        let mut g = HashMap::new();
        let mut seen = HashSet::new();
        pub fn dfs(
            c: char,
            min_n: char,
            g: &HashMap<char, Vec<char>>,
            seen: &mut HashSet<char>,
        ) -> char {
            if seen.contains(&c) {
                return min_n;
            }
            seen.insert(c);
            let mut ans = min_n;
            for &nc in g.get(&c).unwrap_or(&Vec::new()) {
                if !seen.contains(&nc) {
                    ans = ans.min(dfs(nc, min_n.min(nc), g, seen));
                }
            }
            ans
        }
        for (ac, bc) in s1.chars().zip(s2.chars()) {
            g.entry(ac).or_insert(Vec::new()).push(bc);
            g.entry(bc).or_insert(Vec::new()).push(ac);
        }
        base_str
            .chars()
            .map(|c| {
                seen.clear();
                dfs(c, c, &g, &mut seen)
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_smallest_equivalent_string_1() {
        assert_eq!(
            String::from("makkek"),
            Solution::smallest_equivalent_string(
                String::from("parker",),
                String::from("morris"),
                String::from("parser")
            )
        );
    }
    #[test]
    pub fn test_smallest_equivalent_string_2() {
        assert_eq!(
            String::from("hdld"),
            Solution::smallest_equivalent_string(
                String::from("hello",),
                String::from("world"),
                String::from("hold")
            )
        );
    }
    #[test]
    pub fn test_smallest_equivalent_string_3() {
        assert_eq!(
            String::from("aauaaaaada"),
            Solution::smallest_equivalent_string(
                String::from("leetcode",),
                String::from("programs"),
                String::from("sourcecode")
            )
        );
    }
}

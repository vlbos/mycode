// 760\. Find Anagram Mappings
// ===========================

// Given two lists `A`and `B`, and `B` is an anagram of `A`. `B` is an anagram of `A` means `B` is made by randomizing the order of the elements in `A`.

// We want to find an _index mapping_ `P`, from `A` to `B`. A mapping `P[i] = j` means the `i`th element in `A` appears in `B` at index `j`.

// These lists `A` and `B` may contain duplicates. If there are multiple answers, output any of them.

// For example, given

// A = \[12, 28, 46, 32, 50\]
// B = \[50, 12, 32, 46, 28\]

// We should return

// \[1, 4, 3, 2, 0\]

// as `P[0] = 1` because the `0`th element of `A` appears at `B[1]`, and `P[1] = 4` because the `1`st element of `A` appears at `B[4]`, and so on.

// **Note:**

// 1.  `A, B` have equal lengths in range `[1, 100]`.
// 2.  `A[i], B[i]` are integers in range `[0, 10^5]`.

// ### Difficulty:

// Easy

// ### Lock:

// Prime

// ### Company:

// [Google](https://leetcode.ca/tags/#Google)
pub struct Solution {}
impl Solution {
    pub fn anagram_mappings(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        // Vec::new()
        let bm: std::collections::HashMap<i32, i32> =
            b.iter().enumerate().map(|(i, &v)| (v, i as i32)).collect();
        a.into_iter().map(|v| *bm.get(&v).unwrap()).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_anagram_mappings_1() {
        assert_eq!(
            Vec::<i32>::new(),
            Solution::anagram_mappings(Vec::new(), Vec::new())
        );
    }
}

// 444\. Sequence Reconstruction
// =============================

// Check whether the original sequence `org` can be uniquely reconstructed from the sequences in `seqs`.
// The `org` sequence is a permutation of the integers from 1 to n, with 1 ≤ n ≤ 104.
// Reconstruction means building a shortest common supersequence of the sequences in `seqs`
// (i.e., a shortest sequence so that all sequences in `seqs` are subsequences of it).
// Determine whether there is only one sequence that can be reconstructed from `seqs` and it is the `org` sequence.

// **Example 1:**

// **Input:**
// org: \[1,2,3\], seqs: \[\[1,2\],\[1,3\]\]

// **Output:**
// false

// **Explanation:**
// \[1,2,3\] is not the only one sequence that can be reconstructed, because \[1,3,2\] is also a valid sequence that can be reconstructed.

// **Example 2:**

// **Input:**
// org: \[1,2,3\], seqs: \[\[1,2\]\]

// **Output:**
// false

// **Explanation:**
// The reconstructed sequence can only be \[1,2\].

// **Example 3:**

// **Input:**
// org: \[1,2,3\], seqs: \[\[1,2\],\[1,3\],\[2,3\]\]

// **Output:**
// true

// **Explanation:**
// The sequences \[1,2\], \[1,3\], and \[2,3\] can uniquely reconstruct the original sequence \[1,2,3\].

// **Example 4:**

// **Input:**
// org: \[4,1,5,2,6,3\], seqs: \[\[5,2,6,3\],\[4,1,5,2\]\]

// **Output:**
// true

// **UPDATE (2017/1/8):**
// The _seqs_ parameter had been changed to a list of list of strings (instead of a 2d array of strings).
// Please reload the code definition to get the latest changes.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Google](https://leetcode.ca/tags/#Google)

// @lc code=start

impl Solution {
    pub fn sequence_reconstruction(org: Vec<i32>, seqs: Vec<Vec<i32>>) -> bool {
        // use std::collections::{HashMap, HashSet};
        // let mut org_set = org
        //     .iter()
        //     .map(|k| (*k, 0usize))
        //     .collect::<HashMap<i32, usize>>();
        // let mut ins: HashMap<i32, usize> = HashMap::new();
        // let mut outs: HashMap<i32, HashSet<i32>> = HashMap::new();
        // for s in &seqs {
        //     for i in s {
        //         if !org_set.contains_key(i) {
        //             return false;
        //         } else {
        //             *org_set.get_mut(i).unwrap() += 1;
        //         }
        //     }
        //     for j in 1..s.len() {
        //         let i = s[j - 1];
        //         let o = s[j];
        //         let mut is_dup = false;
        //         outs.entry(o)
        //             .and_modify(|v| {
        //                 if !v.contains(&i) {
        //                     v.insert(i);
        //                 } else {
        //                     is_dup = true
        //                 }
        //             })
        //             .or_insert({
        //                 let mut s = HashSet::new();
        //                 s.insert(i);
        //                 s
        //             });
        //         ins.entry(i)
        //             .and_modify(|v| {
        //                 if !is_dup {
        //                     *v = *v + 1;
        //                 }
        //             })
        //             .or_insert(1);
        //         ins.entry(o).or_insert(0);
        //     }
        // }
        // for (_, v) in org_set {
        //     if v == 0 {
        //         return false;
        //     }
        // }
        // for j in 1..org.len() {
        //     let i = org[j - 1];
        //     let o = org[j];
        //     if !outs.contains_key(&o) || !outs[&o].contains(&i) {
        //         return false;
        //     }
        // }
        // while !ins.is_empty() {
        //     let zeros_ins: HashSet<i32> = ins
        //         .iter()
        //         .filter(|(_, c)| **c == 0)
        //         .map(|(i, _)| *i)
        //         .collect::<HashSet<i32>>();
        //     // println!("{:?}", zeros_ins);
        //     // println!("{:?}, {:?}", ins, outs);
        //     if zeros_ins.is_empty() && !seqs.is_empty() {
        //         return false;
        //     }
        //     for i in zeros_ins {
        //         if outs.contains_key(&i) {
        //             for o in &outs[&i] {
        //                 let v = ins.get_mut(&o).unwrap();
        //                 *v -= 1;
        //             }
        //             outs.remove(&i);
        //         }
        //         ins.remove(&i);
        //     }
        // }
        // true
        let n = org.len();
        let mut to_match = n as i32 - 1;
        use std::collections::{HashMap, HashSet};
        let pos: HashMap<i32, usize> = org.iter().enumerate().map(|(i, &v)| (v, i)).collect();
        let mut num = HashSet::new();
        let mut flags = HashSet::new();
        for seq in &seqs {
            num.insert(seq[0]);
            for w in seq.windows(2) {
                let (x, y) = (w[0], w[1]);
                num.insert(y);
                if *pos.get(&x).unwrap() >= *pos.get(&y).unwrap() {
                    return false;
                }
                if !flags.contains(&x) && *pos.get(&x).unwrap() + 1 == *pos.get(&y).unwrap() {
                    flags.insert(x);
                    to_match -= 1;
                }
            }
        }
        num.len() == n && to_match == 0
    }
}
// @lc code=end

#[allow(dead_code)]
pub  struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sequence_reconstruction_1() {
        assert!(!Solution::sequence_reconstruction(
            vec![1, 2, 3],
            vec![vec![1, 2], vec![1, 3]]
        ));
    }

    #[test]
    fn test_sequence_reconstruction_2() {
        assert!(!Solution::sequence_reconstruction(
            vec![1, 2, 3],
            vec![vec![1, 3]]
        ));
    }

    #[test]
    fn test_sequence_reconstruction_3() {
        assert!(!Solution::sequence_reconstruction(
            vec![1, 2, 3],
            vec![vec![1, 2]]
        ));
    }

    #[test]
    fn test_sequence_reconstruction_4() {
        assert!(Solution::sequence_reconstruction(
            vec![1, 2, 3],
            vec![vec![1, 2], vec![1, 3], vec![2, 3]]
        ));
    }

    #[test]
    fn test_sequence_reconstruction_5() {
        assert!(Solution::sequence_reconstruction(
            vec![4, 1, 5, 2, 6, 3],
            vec![vec![5, 2, 6, 3], vec![4, 1, 5, 2]]
        ));
    }
}

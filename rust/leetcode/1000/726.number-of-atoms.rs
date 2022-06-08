/*
 * @lc app=leetcode id=726 lang=rust
 *
 * [726] Number of Atoms
 */

// @lc code=start
impl Solution {
    pub fn count_of_atoms(formula: String) -> String {
       use std::collections::HashMap;
        let mut s:Vec<HashMap<String,i32>> = Vec::new();
        s.push(HashMap::new());
        let mut i = 0;
        let n = formula.len();
        let bf = formula.as_bytes();
        let parse_atom = |i: &mut usize| -> String {
            let mut j = *i;
            let mut ans = String::new();
            ans.push(bf[j] as char);
            j += 1;
            while j < n && (bf[j] as char).is_ascii_lowercase() {
                ans.push(bf[j] as char);
                j+=1;
            }
            *i = j;
            ans
        };
        let parse_num = |i: &mut usize| -> i32 {
            let mut j = *i;
            while j < n && (bf[j] as char).is_ascii_digit() {
                j += 1;
            }
            let ii = *i;
            *i = j;
            if j > ii {
                formula[ii..j].parse::<i32>().unwrap()
            } else {
                1
            }
        };
        while i < n {
            if bf[i] == b'(' {
                i += 1;
                s.push(HashMap::new());
            } else if bf[i] == b')' {
                i += 1;
                let num = parse_num(&mut i);
                let atom_num = s.pop().unwrap();
                let last = s.len() - 1;
                for (atom, v) in &atom_num {
                    s[last]
                        .entry(atom.clone())
                        .and_modify(|x| *x += v * num)
                        .or_insert(v * num);
                }
            } else {
                let atom = parse_atom(&mut i);
                let num = parse_num(&mut i);
                let last = s.len() - 1;
                s[last]
                    .entry(atom.clone())
                    .and_modify(|x| *x += num)
                    .or_insert(num);
            }
        }
        let mut ss = s[0]
            .iter()
            .map(|x| (x.0.clone(), *x.1))
            .collect::<Vec<(String, i32)>>();
        ss.sort();
        ss.iter()
            .map(|x| {
                 if x.1 > 1 {
                   x.0.clone() + x.1.to_string().as_str()
                } else {
                   x.0.clone() 
                }
            })
            .collect()
    }
}
// @lc code=end

/*
 * @lc app=leetcode id=65 lang=rust
 *
 * [65] Valid Number
 */

// @lc code=start
impl Solution {
    pub fn is_number(s: String) -> bool {
        let mut state = 0;
        let n = 9;
        let mut finals = vec![false; n];

        for i in [3, 5, 6, 8] {
            finals[i] = true;
        }
        use std::collections::HashMap;
        let mut transfer: HashMap<i32, HashMap<i32, i32>> = [
            HashMap::from([(0, 0), (1, 1), (2, 6), (3, 2)]),
            HashMap::from([(2, 6), (3, 2)]),
            HashMap::from([(2, 3)]),
            HashMap::from([(0, 8), (2, 3), (4, 4)]),
            HashMap::from([(1, 7), (2, 5)]),
            HashMap::from([(0, 8), (2, 5)]),
            HashMap::from([(0, 8), (2, 6), (3, 3), (4, 4)]),
            HashMap::from([(2, 5)]),
            HashMap::from([(0, 8)]),
        ]
        .iter()
        .enumerate()
        .map(|(i, v)| (i as i32, v.clone()))
        .collect();

        let make = |c: char| match c {
            ' ' => 0,
            '+' | '-' => 1,
            '.' => 3,
            'e' | 'E' => 4,
            _ => {
                if c.is_ascii_digit() {
                    2
                } else {
                    5
                }
            }
        };
        for c in s.chars() {
            state = *transfer.get(&state).unwrap_or(&HashMap::new()).get(&make(c)).unwrap_or(&-1);
            if state < 0 {
                return false;
            }
        }
        finals[state as usize]
    }
}
// @lc code=end

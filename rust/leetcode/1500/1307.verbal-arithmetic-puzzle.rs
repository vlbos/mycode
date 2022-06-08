/*
 * @lc app=leetcode id=1307 lang=rust
 *
 * [1307] Verbal Arithmetic Puzzle
 */

// @lc code=start
impl Solution {
    pub fn is_solvable(words: Vec<String>, result: String) -> bool {
        use std::collections::HashMap;
        let mut weight = HashMap::new();
        let mut lead_zero = std::collections::HashSet::new();
        for word in &words {
            for (i, b) in word.bytes().rev().enumerate() {
                weight
                    .entry(b)
                    .and_modify(|x| *x += 10i32.pow(i as u32))
                    .or_insert(10i32.pow(i as u32));
            }
            if word.len() > 1 {
                lead_zero.insert(word.as_bytes()[0]);
            }
        }
        for (i, b) in result.bytes().rev().enumerate() {
            weight
                .entry(b)
                .and_modify(|x| *x -= 10i32.pow(i as u32))
                .or_insert(-10i32.pow(i as u32));
        }
        if result.len() > 1 {
            lead_zero.insert(result.as_bytes()[0]);
        }
        let mut weight: Vec<(u8, i32)> = weight.into_iter().collect();
        weight.sort_by_key(|x| -x.1.abs());
        let n = weight.len();
        let mut suffix_sum_min = vec![0; n];
        let mut suffix_sum_max = vec![0; n];
        for i in 0..n {
            let mut suffix_pos: Vec<i32> = weight[i..]
                .iter()
                .filter(|x| x.1 > 0)
                .map(|x| x.1)
                .collect();
            suffix_pos.sort();
            let mut suffix_neg: Vec<i32> = weight[i..]
                .iter()
                .filter(|x| x.1 < 0)
                .map(|x| x.1)
                .collect();
            suffix_pos.sort();
            let spn = suffix_pos.len() as i32;
            suffix_sum_min[i] = suffix_pos
                .iter()
                .enumerate()
                .map(|(i, &v)| (spn - 1 - i as i32) * v)
                .sum::<i32>()
                + suffix_neg
                    .iter()
                    .enumerate()
                    .map(|(i, &v)| (9 - i as i32) * v)
                    .sum::<i32>();
            suffix_sum_max[i] = suffix_pos
                .iter()
                .enumerate()
                .map(|(i, &v)| (10 - spn + i as i32) * v)
                .sum::<i32>()
                + suffix_neg
                    .iter()
                    .enumerate()
                    .map(|(i, &v)| (i as i32) * v)
                    .sum::<i32>();
        }
        let lead_zero: Vec<usize> = weight
            .iter()
            .map(|x| if lead_zero.contains(&x.0) { 1 } else { 0 })
            .collect();
        let mut used = vec![false; 10];
        fn dfs(
            pos: usize,
            total: i32,
            lead_zero: &Vec<usize>,
            weight: &Vec<(u8, i32)>,
            suffix_sum_min: &Vec<i32>,
            suffix_sum_max: &Vec<i32>,
            used: &mut Vec<bool>,
        ) -> bool {
            if pos == weight.len() {
                return total == 0;
            }
            if total + suffix_sum_min[pos] > 0 || total + suffix_sum_max[pos] < 0 {
                return false;
            }
            for i in lead_zero[pos]..10 {
                if used[i] {
                    continue;
                }
                used[i] = true;
                let check = dfs(
                    pos + 1,
                    total + weight[pos].1 * i as i32,
                    lead_zero,
                    weight,
                    suffix_sum_min,
                    suffix_sum_max,
                    used,
                );
                used[i] = false;
                if check {
                    return true;
                }
            }
            false
        }
        dfs(
            0,
            0,
            &lead_zero,
            &weight,
            &suffix_sum_min,
            &suffix_sum_max,
            &mut used,
        )
    }
}
// @lc code=end

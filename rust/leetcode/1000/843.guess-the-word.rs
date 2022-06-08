/*
 * @lc app=leetcode id=843 lang=rust
 *
 * [843] Guess the Word
 */

// @lc code=start
/**
 * // This is the Master's API interface.
 * // You should not implement it, or speculate about its implementation
 * struct Master;
 * impl Master {
 *     fn guess(word:String)->int;
 * };
 */

impl Solution {
    pub fn find_secret_word(words: Vec<String>, master: &Master) {
           let n = words.len();
        let mut h = vec![vec![0; n]; n];
        for i in 0..n {
            for j in 0..n {
                let mut matches = 0;
                let wj = words[j].as_bytes();
                for (k, b) in words[i].bytes().enumerate() {
                    if b == wj[k] {
                        matches += 1;
                    }
                }
                h[i][j] = matches;
                h[j][i] = matches;
            }
        }
        let solve = |possible: &Vec<usize>, path: &Vec<usize>| -> usize {
            if possible.len() <= 2 {
                return possible[0];
            }
            let mut ansgrp = possible.clone();
            let mut ans = 0;
            for (i, v) in h.iter().enumerate() {
                if path.contains(&i) {
                    continue;
                }
                let mut groups = vec![Vec::new(); 7];
                for &j in possible {
                    if i != j {
                        groups[h[i][j]].push(j);
                    }
                }
                let max_group = groups.iter().max_by_key(|x| x.len()).unwrap();
                if max_group.len() < ansgrp.len() {
                    ansgrp = max_group.to_vec();
                    ans = i;
                }
            }
            ans
        };
        let mut possible = (0..n).collect::<Vec<usize>>();
        let mut path = Vec::new();
        while !possible.is_empty() {
            let guess = solve(&possible, &path);
            let matches = master.guess(words[guess].clone()) as usize;
            if matches == words[0].len() {
                return;
            }
            possible = possible
                .into_iter()
                .filter(|j| h[guess][*j] == matches)
                .collect::<Vec<usize>>();
            path.push(guess);
        }
        
    }
}
// @lc code=end

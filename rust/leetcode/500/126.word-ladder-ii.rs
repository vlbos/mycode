/*
 * @lc app=leetcode id=126 lang=rust
 *
 * [126] Word Ladder II
 */

// @lc code=start
impl Solution {
    pub fn find_ladders(
        begin_word: String,
        end_word: String,
        word_list: Vec<String>,
    ) -> Vec<Vec<String>> {
        use std::collections::HashMap;
        use std::collections::HashSet;
        let mut ans = Vec::new();
        let mut ws: HashSet<String> = word_list.iter().cloned().collect();
        if !ws.contains(&end_word) {
            return ans;
        }
        ws.remove(&begin_word);
        let mut steps = HashMap::from([(begin_word.clone(), 0)]);
        let mut from = HashMap::from([(begin_word.clone(), HashSet::new())]);
        let mut step = 0;
        let mut found = false;
        let mut q = std::collections::VecDeque::new();
        q.push_back(begin_word.clone());
        let word_len = begin_word.len();

        while !q.is_empty() {
            step += 1;
            let len = q.len();

            for _ in 0..len {
                let curr_word = q.pop_front().unwrap();
                let mut next_word: Vec<u8> = curr_word.bytes().collect();
                for j in 0..word_len {
                    let origin = next_word[j];
                    for c in b'a'..=b'z' {
                        next_word[j] = c;
                        let nw = String::from_utf8(next_word.clone()).unwrap();
                        if *steps.get(&nw).unwrap_or(&0) == step {
                            from.entry(nw.clone())
                                .or_insert(HashSet::new())
                                .insert(curr_word.clone());
                        }
                        if !ws.contains(&nw) {
                            continue;
                        }
                        ws.remove(&nw);
                        q.push_back(nw.clone());
                        from.entry(nw.clone())
                            .or_insert(HashSet::new())
                            .insert(curr_word.clone());
                        steps.insert(nw.clone(), step);
                        if nw == end_word {
                            found = true;
                        }
                    }
                    next_word[j] = origin;
                }
            }
            if found {
                break;
            }
        }
        if found {
            let mut path = vec![end_word.clone()];
            dfs(&end_word, &from, &mut ans, &mut path);
        }
        fn dfs(
            node: &String,
            from: &HashMap<String, HashSet<String>>,
            ans: &mut Vec<Vec<String>>,
            path: &mut Vec<String>,
        ) {
            if from.get(node).unwrap_or(&HashSet::new()).is_empty() {
                let mut tmp = path.clone();
                tmp.reverse();
                ans.push(tmp.clone());
                return;
            }
            for p in from.get(node).unwrap_or(&HashSet::new()) {
                path.push(p.clone());
                dfs(p, from, ans, path);
                path.pop();
            }
        }
        ans
    }
}
// @lc code=end

/*
 * @lc app=leetcode id=127 lang=rust
 *
 * [127] Word Ladder
 */

// @lc code=start
impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        use std::collections::HashMap;
        use std::collections::VecDeque;
        let mut node_num = 0;
        let mut word_id = HashMap::new();
        let mut edge = Vec::new();
        let add_word = |word: &String,
                        node_num: &mut usize,
                        word_id: &mut HashMap<String, usize>,
                        edge: &mut Vec<Vec<usize>>| {
            if !word_id.contains_key(word) {
                word_id.insert(word.clone(), *node_num);
                *node_num += 1;
                edge.push(Vec::new());
            }
        };
        let add_edge = |word: &String,
                        node_num: &mut usize,
                        word_id: &mut HashMap<String, usize>,
                        edge: &mut Vec<Vec<usize>>| {
            add_word(word, node_num, word_id, edge);
            let id1 = *word_id.get(word).unwrap_or(&0);
            let mut wb: Vec<u8> = word.bytes().collect();
            for i in 0..wb.len() {
                let tmp = wb[i];
                wb[i] = b'*';
                let ww = String::from_utf8(wb.clone()).unwrap();
                add_word(&ww, node_num, word_id, edge);
                let id2 = *word_id.get(&ww).unwrap_or(&0);
                edge[id1].push(id2);
                edge[id2].push(id1);
                wb[i] = tmp;
            }
        };
        for word in &word_list {
            add_edge(word, &mut node_num, &mut word_id, &mut edge);
        }
        add_edge(&begin_word, &mut node_num, &mut word_id, &mut edge);
        if !word_id.contains_key(&end_word) {
            return 0;
        }
        let mut dis_begin = vec![i32::MAX; node_num];
        let begin_id = *word_id.get(&begin_word).unwrap_or(&0);
        dis_begin[begin_id] = 0;
        let mut que_begin = VecDeque::new();
        que_begin.push_back(begin_id);

        let mut dis_end = vec![i32::MAX; node_num];
        let end_id = *word_id.get(&end_word).unwrap_or(&0);
        dis_end[end_id] = 0;
        let mut que_end = VecDeque::new();
        que_end.push_back(end_id);
        while !que_begin.is_empty() && !que_end.is_empty() {
            let que_begin_len = que_begin.len();
            for _ in 0..que_begin_len {
                let node_begin = que_begin.pop_front().unwrap();
                if dis_end[node_begin] != i32::MAX {
                    return (dis_begin[node_begin] + dis_end[node_begin]) / 2 + 1;
                }
                for &e in &edge[node_begin] {
                    if dis_begin[e] == i32::MAX {
                        dis_begin[e] = dis_begin[node_begin] + 1;
                        que_begin.push_back(e);
                    }
                }
            }

            let que_end_len = que_end.len();
            for _ in 0..que_end_len {
                let node_end = que_end.pop_front().unwrap();
                if dis_begin[node_end] != i32::MAX {
                    return (dis_begin[node_end] + dis_end[node_end]) / 2 + 1;
                }
                for &e in &edge[node_end] {
                    if dis_end[e] == i32::MAX {
                        dis_end[e] = dis_end[node_end] + 1;
                        que_end.push_back(e);
                    }
                }
            }
        }
        0
    }
}
// @lc code=end

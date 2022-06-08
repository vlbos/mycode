/*
 * @lc app=leetcode id=488 lang=rust
 *
 * [488] Zuma Game
 */

// @lc code=start
impl Solution {
    pub fn find_min_step(board: String, hand: String) -> i32 {
        let mut h: Vec<char> = hand.chars().collect();
        h.sort();
        let mut b: Vec<char> = board.chars().collect();
        let mut visited = std::collections::HashSet::new();
        visited.insert((b.clone(), h.clone()));
        let mut q = std::collections::VecDeque::new();
        q.push_back(((b.clone(), h.clone(), 0)));
        let clean = |nb: &mut Vec<char>| {
            let mut st:Vec<(char,i32)> = Vec::new();
            for  &c in nb.iter() {
                while !st.is_empty() && c != st[st.len() - 1].0 && st[st.len() - 1].1 >= 3 {
                    st.pop();
                }
                if st.is_empty() || c != st[st.len() - 1].0 {
                    st.push((c, 1));
                } else {
                    let last = st.len() - 1;
                    st[last].1 += 1;
                }
            }

            if !st.is_empty() && st[st.len() - 1].1 >= 3 {
                st.pop();
            }
            nb.clear();
            for i in 0..st.len() {
                for j in 0..st[i].1 {
                    nb.push(st[i].0);
                }
            }
        };
        while let Some((b, h, step)) = q.pop_front() {
            for j in 0..h.len() {
                if j > 0 && h[j] == h[j - 1] {
                    continue;
                }
                for i in 0..=b.len() {
                    if i > 0 && b[i - 1] == h[j] {
                        continue;
                    }
                    let mut choose = false;
                    if i < b.len() && b[i] == h[j] {
                        choose = true;
                    }
                    if i > 0 && i < b.len() && b[i - 1] == b[i] && b[i] != h[j] {
                        choose = true;
                    }
                    if choose {
                        let mut nb = b.clone();
                        let mut nh = h.clone();
                        nb.insert(i, h[j]);
                        clean(&mut nb);
                        nh.remove(j);
                        if nb.is_empty() {
                            return step + 1;
                        }
                        if !visited.contains(&(nb.clone(), nh.clone())) {
                            q.push_back((nb.clone(), nh.clone(), step + 1));
                            visited.insert((nb.clone(), nh.clone()));
                        }
                    }
                }
            }
        }
        -1
    }
}
// @lc code=end

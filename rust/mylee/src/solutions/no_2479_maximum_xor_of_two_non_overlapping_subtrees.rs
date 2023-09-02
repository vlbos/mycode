// # [2479. Maximum XOR of Two Non-Overlapping Subtrees](https://leetcode.com/problems/maximum-xor-of-two-non-overlapping-subtrees)
// ## Description

//  There is an undirected tree with  n  nodes labeled from  0  to  n - 1 .
// You are given the integer  n  and a 2D integer array  edges  of length  n - 1 ,
// where  edges[i] = [a i , b i ]  indicates that there is an edge between nodes  a i   and  b i   in the tree.
// The root of the tree is the node labeled  0 .

//  Each node has an associated  value .
// You are given an array  values  of length  n , where  values[i]  is the  value  of the  i th   node.

//  Select any two  non-overlapping  subtrees.
//  Your  score  is the bitwise XOR of the sum of the values within those subtrees.

//  Return  the    maximum    possible  score  you can achieve .
// If it is impossible to find two nonoverlapping subtrees , return  0 .

//   Note  that:

// 	 The  subtree  of a node is the tree consisting of that node and all of its descendants.
// 	 Two subtrees are  non-overlapping  if they do not share  any common  node.

//  Example 1:
// <img alt="" src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2400-2499/2479.Maximum%20XOR%20of%20Two%20Non-Overlapping%20Subtrees/images/treemaxxor.png" style="width: 346px; height: 249px;" />

//  Input:  n = 6, edges = [[0,1],[0,2],[1,3],[1,4],[2,5]], values = [2,8,3,6,2,5]
//  Output:  24
//  Explanation:  Node 1'tree subtree has sum of values 16, while node 2'tree subtree has sum of values 8,
// so choosing these nodes will yield a score of 16 XOR 8 = 24.
// It can be proved that is the maximum possible score we can obtain.

//  Example 2:
// <img alt="" src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2400-2499/2479.Maximum%20XOR%20of%20Two%20Non-Overlapping%20Subtrees/images/tree3drawio.png" style="width: 240px; height: 261px;" />

//  Input:  n = 3, edges = [[0,1],[1,2]], values = [4,6,1]
//  Output:  0
//  Explanation:  There is no possible way to select two non-overlapping subtrees, so we just return 0.

//   Constraints:

// 	  2 <= n <= 5 * 10^4
// 	  edges.length == n - 1
// 	  0 <= a i , b i  < n
// 	  values.length == n
// 	  1 <= values[i] <= 10^9
// 	 It is guaranteed that  edges  represents a valid tree.
//    long long max_xor(int n, vector<vector<int>>& edges, vector<int>& values) {
use std::collections::HashMap;
struct Trie {
    children: HashMap<i64, Trie>,
}
impl Trie {
    pub fn new() -> Self {
        Self {
            children: HashMap::new(),
        }
    }
    pub fn insert(&mut self, x: i64) {
        let mut node = self;
        for i in (0..48).rev() {
            let v = (x >> i) & 1;
            node = node.children.entry(v).or_insert(Trie::new());
        }
    }
    pub fn search(&self, x: i64) -> i64 {
        let mut node = self;
        let mut ans = 0;
        for i in (0..48).rev() {
            let v = (x >> i) & 1;
            if node.children.is_empty() {
                return ans;
            };
            if let Some(child) = node.children.get(&(v ^ 1)) {
                ans = ans << 1 | 1;
                node = child;
            } else {
                ans <<= 1;
                if let Some(child) = node.children.get(&v) {
                    node = child;
                }
            }
        }
        ans
    }
}
#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn max_xor(_n: i32, edges: Vec<Vec<i32>>, values: Vec<i32>) -> i64 {
        fn dfs1(
            i: i32,
            fa: i32,
            values: &Vec<i32>,
            g: &HashMap<i32, Vec<i32>>,
            s: &mut HashMap<i32, i64>,
        ) -> i64 {
            let mut t = values[i as usize] as i64;
            for &j in g.get(&i).unwrap_or(&Vec::new()) {
                if j != fa {
                    t += dfs1(j, i, values, g, s);
                }
            }
            s.insert(i, t);
            t
        }
        fn dfs2(
            i: i32,
            fa: i32,
            g: &HashMap<i32, Vec<i32>>,
            s: &HashMap<i32, i64>,
            tree: &mut Trie,
            ans: &mut i64,
        ) {
            *ans = (*ans).max(tree.search(s[&i]));
            for &j in g.get(&i).unwrap_or(&Vec::new()) {
                if j != fa {
                    dfs2(j, i, g, s, tree, ans);
                }
            }
            tree.insert(s[&i]);
        }
        let mut g = HashMap::new();
        for e in &edges {
            g.entry(e[0]).or_insert(Vec::new()).push(e[1]);
            g.entry(e[1]).or_insert(Vec::new()).push(e[0]);
        }
        let mut s = HashMap::new();
        dfs1(0, -1, &values, &g, &mut s);
        let mut tree = Trie::new();
        let mut ans = 0;
        dfs2(0, -1, &g, &s, &mut tree, &mut ans);
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_max_xor_1() {
        assert_eq!(
            24,
            Solution::max_xor(
                6,
                vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5]],
                vec![2, 8, 3, 6, 2, 5]
            )
        );
    }
    #[test]
    pub fn test_max_xor_2() {
        assert_eq!(
            0,
            Solution::max_xor(3, vec![vec![0, 1], vec![1, 2]], vec![4, 6, 1])
        );
    }
}

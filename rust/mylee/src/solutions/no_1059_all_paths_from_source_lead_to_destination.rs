// 1059\. All Paths from Source Lead to Destination
// ================================================

// Given the `edges` of a directed graph, and two nodes `source` and `destination` of this graph,
// determine whether or not all paths starting from `source` eventually end at `destination`, that is:

// *   At least one path exists from the `source` node to the `destination` node
// *   If a path exists from the `source` node to a node with no outgoing edges, then that node is equal to `destination`.
// *   The number of possible paths from `source` to `destination` is a finite number.

// Return `true` if and only if all roads from `source` lead to `destination`.

// **Example 1:**

// ![](https://leetcode.ca/all/img/1059_1.jpg)

// **Input:** n = 3, edges = \[\[0,1\],\[0,2\]\], source = 0, destination = 2
// **Output:** false
// **Explanation:** It is possible to reach and get stuck on both node 1 and node 2.

// **Example 2:**

// ![](https://leetcode.ca/all/img/1059_2.jpg)

// **Input:** n = 4, edges = \[\[0,1\],\[0,3\],\[1,2\],\[2,1\]\], source = 0, destination = 3
// **Output:** false
// **Explanation:** We have two possibilities: to end at node 3, or to loop over node 1 and node 2 indefinitely.

// **Example 3:**

// ![](https://leetcode.ca/all/img/1059_3.jpg)

// **Input:** n = 4, edges = \[\[0,1\],\[0,2\],\[1,3\],\[2,3\]\], source = 0, destination = 3
// **Output:** true

// **Example 4:**

// ![](https://leetcode.ca/all/img/1059_4.jpg)

// **Input:** n = 3, edges = \[\[0,1\],\[1,1\],\[1,2\]\], source = 0, destination = 2
// **Output:** false
// **Explanation:** All paths from the source node end at the destination node, but there are an infinite number of paths, such as 0-1-2, 0-1-1-2, 0-1-1-1-2, 0-1-1-1-1-2, and so on.

// **Example 5:**

// ![](https://leetcode.ca/all/img/1059_5.jpg)

// **Input:** n = 2, edges = \[\[0,1\],\[1,1\]\], source = 0, destination = 1
// **Output:** false
// **Explanation:** There is infinite self-loop at destination node.

// **Note:**

// 1.  The given graph may have self loops and parallel edges.
// 2.  The number of nodes `n` in the graph is between `1` and `10000`
// 3.  The number of edges in the graph is between `0` and `10000`
// 4.  `0 <= edges.length <= 10000`
// 5.  `edges[i].length == 2`
// 6.  `0 <= source <= n - 1`
// 7.  `0 <= destination <= n - 1`

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Google](https://leetcode.ca/tags/#Google)

#[allow(dead_code)]
pub  struct Solution {}
impl Solution {
    pub fn leads_to_destination(
        _n: i32,
        edges: Vec<Vec<i32>>,
        source: i32,
        destination: i32,
    ) -> bool {
        use std::collections::{HashMap, HashSet};
        let mut g = HashMap::new();
        for e in &edges {
            g.entry(e[0]).or_insert(Vec::new()).push(e[1]);
        }
        fn dfs(g: &HashMap<i32, Vec<i32>>, visited: &mut HashSet<i32>, cur: i32, end: i32) -> bool {
            if !g.contains_key(&cur) {
                return cur == end;
            }
            visited.insert(cur);
            for &neighbor in g.get(&cur).unwrap_or(&Vec::new()) {
                if visited.contains(&neighbor) || !dfs(g, visited, neighbor, end) {
                    return false;
                }
            }
            visited.remove(&cur);
            true
        }
        let mut visited = HashSet::new();
        dfs(&g, &mut visited, source, destination)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_leads_to_destination_1() {
        assert!(!Solution::leads_to_destination(
            3,
            vec![vec![0, 1], vec![0, 2]],
            0,
            2
        ));
    }
    #[test]
    fn test_leads_to_destination_2() {
        assert!(!Solution::leads_to_destination(
            4,
            vec![vec![0, 1], vec![0, 3], vec![1, 2], vec![2, 1]],
            0,
            3
        ));
    }
    #[test]
    fn test_leads_to_destination_3() {
        assert!(Solution::leads_to_destination(
            4,
            vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![2, 3]],
            0,
            3
        ));
    }
    #[test]
    fn test_leads_to_destination_4() {
        assert!(!Solution::leads_to_destination(
            3,
            vec![vec![0, 1], vec![1, 1], vec![1, 2]],
            0,
            2
        ));
    }
    #[test]
    fn test_leads_to_destination_5() {
        assert!(!Solution::leads_to_destination(
            2,
            vec![vec![0, 1], vec![1, 1]],
            0,
            1
        ));
    }
}

// # [2307. Check for Contradictions in Equations](https://leetcode.com/problems/check-for-contradictions-in-equations)

// ## Description

// You are given a 2D array of strings equations and an array of real numbers values,
// where equations[i] = [Ai, Bi] and values[i] means that Ai / Bi = values[i].

// Determine if there exists a contradiction in the equations. Return true if there is a contradiction, or false otherwise.

// Note:

//
// 	When checking if two numbers are equal, check that their absolute difference is less than 10-5.
// 	The testcases are generated such that there are no cases targeting precision, i.e. using double is enough to solve the problem.
//

// Example 1:

//
// Input: equations = [["a","b"],["b","c"],["a","c"]], values = [3,0.5,1.5]
// Output: false
// Explanation:
// The given equations are: a / b = 3, b / c = 0.5, a / c = 1.5
// There are no contradictions in the equations. One possible assignment to satisfy all equations is:
// a = 3, b = 1 and c = 2.
//

// Example 2:

//
// Input: equations = [["le","et"],["le","code"],["code","et"]], values = [2,5,0.5]
// Output: true
// Explanation:
// The given equations are: le / et = 2, le / code = 5, code / et = 0.5
// Based on the first two equations, we get code / et = 0.4.
// Since the third equation is code / et = 0.5, we get a contradiction.
//

// Constraints:

//
// 	1 <= equations.length <= 100
// 	equations[i].length == 2
// 	1 <= Ai.length, Bi.length <= 5
// 	Ai, Bi consist of lowercase English letters.
// 	equations.length == values.length
// 	0.0 < values[i] <= 10.0
// 	values[i] has a maximum of 2 decimal places.
//

//   bool check_contradictions(vector<vector<string>>& equations,
//                            vector<double>& values) {

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn check_contradictions(equations: Vec<Vec<String>>, values: Vec<f64>) -> bool {
        let mut m = std::collections::HashMap::new();
        for e in &equations {
            for u in e {
                let len = m.len();
                m.entry(u).or_insert(len);
            }
        }
        let mut g = vec![Vec::new(); m.len()];
        let mut seen = vec![0.0; g.len()];
        for (i, e) in equations.iter().enumerate() {
            let (u, v) = (
                *m.get(&e[0]).unwrap() as usize,
                *m.get(&e[1]).unwrap() as usize,
            );
            g[u].push((v, values[i]));
            g[v].push((u, 1.0 / values[i]));
        }
        fn dfs(u: usize, val: f64, g: &Vec<Vec<(usize, f64)>>, seen: &mut Vec<f64>) -> bool {
            if seen[u] > 0.00001 {
                return (val / seen[u] - 1.0).abs() > 0.00001;
            }
            seen[u] = val;
            for &(v, w) in &g[u] {
                if dfs(v, val / w, g, seen) {
                    return true;
                }
            }
            false
        }
        for (i, e) in g.iter().enumerate() {
            if seen[i] < 0.00001 && dfs(i, 1.0, &g, &mut seen) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_check_contradictions_1() {
        assert!(!Solution::check_contradictions(
            [["a", "b"], ["b", "c"], ["a", "c"]]
                .into_iter()
                .map(|x| x.into_iter().map(String::from).collect::<Vec<String>>())
                .collect::<Vec<Vec<String>>>(),
            vec![3.0, 0.5, 1.5]
        ));
    }
    #[test]
    pub fn test_check_contradictions_2() {
        assert!(Solution::check_contradictions(
            [["le", "et"], ["le", "code"], ["code", "et"]]
                .into_iter()
                .map(|x| x.into_iter().map(String::from).collect::<Vec<String>>())
                .collect::<Vec<Vec<String>>>(),
            vec![2.0, 5.0, 0.5]
        ));
    }
}

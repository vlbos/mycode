// ## [3437\. Permutations III 🔒](https://leetcode.com/problems/permutations-iii)

// [![](https://img.shields.io/badge/Difficulty-Medium-4051B5?style=flat-square)](https://img.shields.io/badge/Difficulty-Medium-4051B5?style=flat-square)

// ## Description

// Given an integer `n`, an **alternating permutation** is a permutation of the first `n` positive integers such that no **two** adjacent elements are **both** odd or **both** even.

// Return *all such* **alternating permutations** sorted in lexicographical order.

// **Example 1:**

// **Input:** n = 4

// **Output:** \[\[1,2,3,4\],\[1,4,3,2\],\[2,1,4,3\],\[2,3,4,1\],\[3,2,1,4\],\[3,4,1,2\],\[4,1,2,3\],\[4,3,2,1\]\]

// **Example 2:**

// **Input:** n = 2

// **Output:** \[\[1,2\],\[2,1\]\]

// **Example 3:**

// **Input:** n = 3

// **Output:** \[\[1,2,3\],\[3,2,1\]\]

// **Constraints:**

// +   `1 <= n <= 10`

//    vector<vector<int>> permute(int n) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn permute(n: i32) -> Vec<Vec<i32>> {
        fn dfs(i:i32,n:i32,vis:&mut Vec<bool>,t:&mut Vec<i32>,ans:&mut Vec<Vec<i32>>){
            if i>=n{
                ans.push(t.clone());
                return 
            }
            for j in 1..=n{
                if !vis[j as usize] && (i==0|| t[t.len()-1]&1!=j&1){
                    t.push(j);
                    vis[j as usize]=true;
                    dfs(i+1,n,vis,t,ans);
                    vis[j as usize]=false;
                    t.pop();
                }
            }
        }
        let mut ans=vec![];
        dfs(0,n,&mut vec![false;n as usize+1],&mut vec![],&mut ans);
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lc_matrix;
    #[test]
    pub fn test_permute_1() {
        assert_eq!(
            lc_matrix![
                [1, 2, 3, 4],
                [1, 4, 3, 2],
                [2, 1, 4, 3],
                [2, 3, 4, 1],
                [3, 2, 1, 4],
                [3, 4, 1, 2],
                [4, 1, 2, 3],
                [4, 3, 2, 1]
            ],
            Solution::permute(4)
        );
    }
    #[test]
    pub fn test_permute_2() {
        assert_eq!(lc_matrix![[1, 2], [2, 1]], Solution::permute(2));
    }
    #[test]
    pub fn test_permute_3() {
        assert_eq!(lc_matrix![[1, 2, 3], [3, 2, 1]], Solution::permute(3));
    }
}

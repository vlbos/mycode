// # [277. Find the Celebrity](https://leetcode.com/problems/find-the-celebrity)

// ## Description

// Suppose you are at a party with n people labeled from 0 to n - 1 and among them, there may exist one celebrity. The definition of a celebrity is that all the other n - 1 people know the celebrity, but the celebrity does not know any of them.

// Now you want to find out who the celebrity is or verify that there is not one. The only thing you are allowed to do is ask questions like: "Hi, A. Do you know B?" to get information about whether A knows B. You need to find out the celebrity (or verify there is not one) by asking as few questions as possible (in the asymptotic sense).

// You are given a helper function bool knows(a, b) that tells you whether A knows B. Implement a function int findCelebrity(n). There will be exactly one celebrity if they are at the party.

// Return the celebrity&#39;s label if there is a celebrity at the party. If there is no celebrity, return -1.


// Example 1:
// <img alt="" src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/0200-0299/0277.Find%20the%20Celebrity/images/g1.jpg" style="width: 224px; height: 145px;" />
// 
// Input: graph = [[1,1,0],[0,1,0],[1,1,1]]
// Output: 1
// Explanation: There are three persons labeled with 0, 1 and 2. graph[i][j] = 1 means person i knows person j, otherwise graph[i][j] = 0 means person i does not know person j. The celebrity is the person labeled as 1 because both 0 and 2 know him but 1 does not know anybody.
// 

// Example 2:
// <img alt="" src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/0200-0299/0277.Find%20the%20Celebrity/images/g2.jpg" style="width: 224px; height: 145px;" />
// 
// Input: graph = [[1,0,1],[1,1,0],[0,1,1]]
// Output: -1
// Explanation: There is no celebrity.
// 


// Constraints:

// 
// 	n == graph.length == graph[i].length
// 	2 <= n <= 100
// 	graph[i][j] is 0 or 1.
// 	graph[i][i] == 1
// 


// Follow up: If the maximum number of allowed calls to the API knows is 3 * n, could you find a solution without exceeding the maximum number of calls?

/* The knows API is defined for you.
       knows(a: i32, b: i32)->bool;
    to call it use self.knows(a,b)
*/

impl Solution {
    pub fn find_celebrity(&self, n: i32) -> i32 {
        let mut ans = 0;

        for i in 0..n {
            if self.knows(ans, i) {
                ans = i;
            }
        }

        for i in 0..n {
            if ans != i {
                if !self.knows(i, ans) || self.knows(ans,i) {
                    return -1;
                }
            }
        }

        ans
    }
    fn  knows(&self,a: i32, b: i32)->bool{  
    true}
}
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::tree;
    #[test]
    pub fn test_read() {
        // assert_eq!(
        //     Solution::read(tree![1, 2, 3, 4, 5]),
        //     tree![4, 5, 2, null, null, 3, 1]
        // );
    }
}
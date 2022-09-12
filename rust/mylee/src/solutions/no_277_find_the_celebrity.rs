// # [277. Find the Celebrity](https://leetcode.com/problems/find-the-celebrity)

// ## Description

// <p>Suppose you are at a party with <code>n</code> people labeled from <code>0</code> to <code>n - 1</code> and among them, there may exist one celebrity. The definition of a celebrity is that all the other <code>n - 1</code> people know the celebrity, but the celebrity does not know any of them.</p>

// <p>Now you want to find out who the celebrity is or verify that there is not one. The only thing you are allowed to do is ask questions like: &quot;Hi, A. Do you know B?&quot; to get information about whether A knows B. You need to find out the celebrity (or verify there is not one) by asking as few questions as possible (in the asymptotic sense).</p>

// <p>You are given a helper function <code>bool knows(a, b)</code> that tells you whether A knows B. Implement a function <code>int findCelebrity(n)</code>. There will be exactly one celebrity if they are at the party.</p>

// <p>Return <em>the celebrity&#39;s label if there is a celebrity at the party</em>. If there is no celebrity, return <code>-1</code>.</p>

// <p>&nbsp;</p>
// <p><strong>Example 1:</strong></p>
// <img alt="" src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/0200-0299/0277.Find%20the%20Celebrity/images/g1.jpg" style="width: 224px; height: 145px;" />
// <pre>
// <strong>Input:</strong> graph = [[1,1,0],[0,1,0],[1,1,1]]
// <strong>Output:</strong> 1
// <strong>Explanation:</strong> There are three persons labeled with 0, 1 and 2. graph[i][j] = 1 means person i knows person j, otherwise graph[i][j] = 0 means person i does not know person j. The celebrity is the person labeled as 1 because both 0 and 2 know him but 1 does not know anybody.
// </pre>

// <p><strong>Example 2:</strong></p>
// <img alt="" src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/0200-0299/0277.Find%20the%20Celebrity/images/g2.jpg" style="width: 224px; height: 145px;" />
// <pre>
// <strong>Input:</strong> graph = [[1,0,1],[1,1,0],[0,1,1]]
// <strong>Output:</strong> -1
// <strong>Explanation:</strong> There is no celebrity.
// </pre>

// <p>&nbsp;</p>
// <p><strong>Constraints:</strong></p>

// <ul>
// 	<li><code>n == graph.length == graph[i].length</code></li>
// 	<li><code>2 &lt;= n &lt;= 100</code></li>
// 	<li><code>graph[i][j]</code> is <code>0</code> or <code>1</code>.</li>
// 	<li><code>graph[i][i] == 1</code></li>
// </ul>

// <p>&nbsp;</p>
// <p><strong>Follow up:</strong> If the maximum number of allowed calls to the API <code>knows</code> is <code>3 * n</code>, could you find a solution without exceeding the maximum number of calls?</p>

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
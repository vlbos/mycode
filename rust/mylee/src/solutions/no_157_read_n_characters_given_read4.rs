// # [157. Read N Characters Given Read4](https://leetcode.com/problems/read-n-characters-given-read4)

// ## Description

// Given a file and assume that you can only read the file using a given method read4, implement a method to read n characters.

// Method read4:

// The API read4 reads four consecutive characters from file, then writes those characters into the buffer array buf4.

// The return value is the number of actual characters read.

// Note that read4() has its own file pointer, much like FILE *fp in C.

// Definition of read4:

//
//     Parameter:  char[] buf4
//     Returns:    int

// buf4[] is a destination, not a source. The results from read4 will be copied to buf4[].
//

// Below is a high-level example of how read4 works:
// <img alt="" src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/0100-0199/0157.Read%20N%20Characters%20Given%20Read4/images/157_example.png" style="width: 600px; height: 403px;" />
//
// File file("abcde"); // File is "abcde", initially file pointer (fp) points to &#39;a&#39;
// char[] buf4 = new char[4]; // Create buffer with enough space to store characters
// read4(buf4); // read4 returns 4. Now buf4 = "abcd", fp points to &#39;e&#39;
// read4(buf4); // read4 returns 1. Now buf4 = "e", fp points to end of file
// read4(buf4); // read4 returns 0. Now buf4 = "", fp points to end of file
//

// Method read:

// By using the read4 method, implement the method read that reads n characters from file and store it in the buffer array buf. Consider that you cannot manipulate file directly.

// The return value is the number of actual characters read.

// Definition of read:

//
//     Parameters:	char[] buf, int n
//     Returns:	int

// buf[] is a destination, not a source. You will need to write the results to buf[].
//

// Note:

//
// 	Consider that you cannot manipulate the file directly. The file is only accessible for read4 but not for read.
// 	The read function will only be called once for each test case.
// 	You may assume the destination buffer array, buf, is guaranteed to have enough space for storing n characters.
//

// Example 1:

//
// Input: file = "abc", n = 4
// Output: 3
// Explanation: After calling your read method, buf should contain "abc". We read a total of 3 characters from the file, so return 3.
// Note that "abc" is the file&#39;s content, not buf. buf is the destination buffer that you will have to write the results to.
//

// Example 2:

//
// Input: file = "abcde", n = 5
// Output: 5
// Explanation: After calling your read method, buf should contain "abcde". We read a total of 5 characters from the file, so return 5.
//

// Example 3:

//
// Input: file = "abcdABCD1234", n = 12
// Output: 12
// Explanation: After calling your read method, buf should contain "abcdABCD1234". We read a total of 12 characters from the file, so return 12.
//

// Constraints:

//
// 	1 <= file.length <= 500
// 	file consist of English letters and digits.
// 	1 <= n <= 1000
//

/**
 * The read4 API is defined as.
 *     fn read4(&self,buf4: &mut [char]) -> i32;
 * You can call it using self.read4(buf4)
 */

impl Solution {
    pub fn read(&self, buf: &mut [char], n: i32) -> i32 {
        let mut ans = 0;
        while ans < n {
            let mut tmp = vec![' '; 4];
            let num = self.read4(&mut tmp);

            if num == 0 {
                return ans;
            }

            for j in 0..num {
                if ans >= n {
                    return ans;
                }

                buf[ans as usize] = tmp[j as usize];
                ans += 1;
            }
        }

        ans
    }
    fn read4(&self, buf4: &mut [char]) -> i32 {
        0
    }
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

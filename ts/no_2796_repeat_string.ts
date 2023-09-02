// # [2796. Repeat String](https://leetcode.com/problems/repeat-string)

// ## Description

// Write code that enhances all strings such that you can call the string.
// replicate(x) method on any string and it will return repeated string x times.

// Try to implement it without using the built-in method string.repeat.

 
//  ### Example 1:


// Input: str = "hello", times = 2
// Output: "hellohello"
// Explanation: "hello" is repeated 2 times


//  ### Example 2:


// Input: str = "code", times = 3
// Output: "codecodecode"
// Explanation: "code" is repeated 3 times


//  ### Example 3:


// Input: str = "js", times = 1
// Output: "js"
// Explanation: "js" is repeated 1 time


 
// Constraints:


// 	1 <= str.length <= 1000
// 	1 <= times <= 1000


// ## Solutions

// <!-- tabs:start -->

// ### **TypeScript**

// ```ts
// declare global {
//     interface String {
//         replicate(times: number): string;
//     }
// }

// String.prototype.replicate = function (times: number) {
//     return new Array(times).fill(this).join('');
// };
// ```

// ### **JavaScript**

// ```js
// String.prototype.replicate = function (times) {
//     return Array(times).fill(this).join('');
// };
// ```

// <!-- tabs:end -->

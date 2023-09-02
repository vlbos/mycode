// # [2823. Deep Object Filter](https://leetcode.com/problems/deep-object-filter)

// ## Description

// Given an object obj and a function fn, return a filtered object filteredObject. 

// Function deepFilter should perform a deep filter operation on the object obj. 
// The deep filter operation should remove properties for which the output of the filter function fn is false, 
// as well as any empty objects or arrays that remain after the keys have been removed.

// If the deep filter operation results in an empty object or array, with no remaining properties, 
// deepFilter should return undefined to indicate that there is no valid data left in the filteredObject.

 
// ### Example 1:


// Input: 
// obj = [-5, -4, -3, -2, -1, 0, 1], 
// fn = (x) = > x  > 0
// Output: [1]
// Explanation: All values that were not greater than 0 were removed.


// ### Example 2:


// Input: 
// obj = {"a": 1, "b": "2", "c": 3, "d": "4", "e": 5, "f": 6, "g": {"a": 1}}, 
// fn = (x) = > typeof x === "string"
// Output: {"b":"2","d":"4"}
// Explanation: All keys with values that were not a string were removed. 
// When the object keys were removed during the filtering process, any resulting empty objects were also removed.


// ### Example 3:


// Input: 
// obj = [-1, [-1, -1, 5, -1, 10], -1, [-1], [-5]], 
// fn = (x) = > x  > 0
// Output: [[5,10]]
// Explanation: All values that were not greater than 0 were removed. 
// When the values were removed during the filtering process, any resulting empty arrays were also removed.

// ### Example 4:


// Input: 
// obj = [[[[5]]]], 
// fn = (x) = > Array.isArray(x)
// Output: undefined


 
// Constraints:


// 	fn is a function that returns a boolean value
// 	obj is a valid JSON object
// 	2 <= JSON.stringify(obj).length <= 10**5


// ## Solutions

// <!-- tabs:start -->

// ### **TypeScript**

// ```ts
// function deepFilter(
//     obj: Record<string, any>,
//     fn: Function,
// ): Record<string, any> | undefined {
//     const dfs = (data: any): any => {
//         if (Array.isArray(data)) {
//             const res = data.map(dfs).filter((item: any) => item !== undefined);
//             return res.length > 0 ? res : undefined;
//         }
//         if (typeof data === 'object' && data !== null) {
//             const res: Record<string, any> = {};
//             for (const key in data) {
//                 if (data.hasOwnProperty(key)) {
//                     const filteredValue = dfs(data[key]);
//                     if (filteredValue !== undefined) {
//                         res[key] = filteredValue;
//                     }
//                 }
//             }
//             return Object.keys(res).length > 0 ? res : undefined;
//         }
//         return fn(data) ? data : undefined;
//     };

//     return dfs(obj);
// }
// ```

// <!-- tabs:end -->

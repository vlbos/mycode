// # [2775. Undefined to Null](https://leetcode.com/problems/undefined-to-null)


// ## Description

// Write a function called undefinedToNull that takes a deeply nested object or array obj, 
// and then creates a copy of that object with any undefined values replaced by null.

// undefined values are handled differently than null values when objects are converted to a JSON string using JSON.stringify(). 
// This function helps ensure serialized data is free of unexpected errors.

 
// ### Example 1:


// Input: obj = {"a": undefined, "b": 3}
// Output: {"a": null, "b": 3}
// Explanation: The value for obj.a has been changed from undefined to null


// ### Example 2:


// Input: obj = {"a": undefined, "b": ["a", undefined]}
// Output: {"a": null,"b": ["a", null]}
// Explanation: The values for obj.a and obj.b[1] have been changed from undefined to null


 
// Constraints:


// 	2  <= JSON.stringify(obj).length  <= 105


// ## Solutions

// <!-- tabs:start -->

// ### **TypeScript**

// ```ts
// function undefinedToNull(obj: Record<any, any>): Record<any, any> {
//     for (const key in obj) {
//         if (typeof obj[key] === 'object') {
//             obj[key] = undefinedToNull(obj[key]);
//         }
//         if (obj[key] === undefined) {
//             obj[key] = null;
//         }
//     }
//     return obj;
// }

// /**
//  * undefinedToNull({"a": undefined, "b": 3}) // {"a": null, "b": 3}
//  * undefinedToNull([undefined, undefined]) // [null, null]
//  */
// ```

// <!-- tabs:end -->

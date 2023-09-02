// # [2692. Make Object Immutable](https://leetcode.com/problems/make-object-immutable)

 

// ## Description

// Write a function that takes an object obj and returns a new immutable version of this object.

// An immutable object is an object that can't be altered and will throw an error if any attempt is made to alter it.

// There are three types of error messages that can be produced from this new object.


// 	Attempting to modify a key on the object will result in this error message: `Error Modifying: ${key}`.
// 	Attempting to modify an index on an array will result in this error message: `Error Modifying Index: ${index}`.
// 	Attempting to call a method that mutates an array will result in this error message: `Error Calling Method: ${methodName}`. 
// You may assume the only methods that can mutate an array are ['pop', 'push', 'shift', 'unshift', 'splice', 'sort', 'reverse'].


// obj is a valid JSON object or array, meaning it is the output of JSON.parse().

// Note that a string literal should be thrown, not an Error.

 
//  ### Example 1:


// Input: 
// obj = {
//   "x": 5
// }
// fn = (obj) => { 
//   obj.x = 5;
//   return obj.x;
// }
// Output: {"value": null, "error": "Error Modifying: x"}
// Explanation: Attempting to modify a key on an object resuts in a thrown error.
//  Note that it doesn't matter that the value was set to the same value as it was before.


//  ### Example 2:


// Input: 
// obj = [1, 2, 3]
// fn = (arr) => { 
//   arr[1] = {}; 
//   return arr[2]; 
// }
// Output: {"value": null, "error": "Error Modifying Index: 1"}
// Explanation: Attempting to modify an array results in a thrown error.


//  ### Example 3:


// Input: 
// obj = {
//   "arr": [1, 2, 3]
// }
// fn = (obj) => { 
//   obj.arr.push(4);
//   return 42;
// }
// Output: { "value": null, "error": "Error Calling Method: push"}
// Explanation: Calling a method that can result in a mutation results in a thrown error.


//  ### Example 4:


// Input: 
// obj = {
//   "x": 2,
//   "y": 2
// }
// fn = (obj) => { 
//   return Object.keys(obj);
// }
// Output: {"value": ["x", "y"], "error": null}
// Explanation: No mutations were attempted so the function returns as normal.


 
// Constraints:


// 	2  <= JSON.stringify(obj).length  <= 105


// ## Solutions

// <!-- tabs:start -->

// ### **TypeScript**

// ```ts
// type Obj = Array | Record<any, any>;

// function makeImmutable(obj: Obj): Obj {
//     const arrayHandler: ProxyHandler<Array> = {
//         set: (_, prop) => {
//             throw `Error Modifying Index: ${String(prop)}`;
//         },
//     };
//     const objectHandler: ProxyHandler<Record<any, any>> = {
//         set: (_, prop) => {
//             throw `Error Modifying: ${String(prop)}`;
//         },
//     };
//     const fnHandler: ProxyHandler = {
//         apply: target => {
//             throw `Error Calling Method: ${target.name}`;
//         },
//     };
//     const fn = ['pop', 'push', 'shift', 'unshift', 'splice', 'sort', 'reverse'];
//     const dfs = (obj: Obj) => {
//         for (const key in obj) {
//             if (typeof obj[key] === 'object' && obj[key] !== null) {
//                 obj[key] = dfs(obj[key]);
//             }
//         }
//         if (Array.isArray(obj)) {
//             fn.forEach(f => (obj[f] = new Proxy(obj[f], fnHandler)));
//             return new Proxy(obj, arrayHandler);
//         }
//         return new Proxy(obj, objectHandler);
//     };
//     return dfs(obj);
// }

// /**
//  * const obj = makeImmutable({x: 5});
//  * obj.x = 6; // throws "Error Modifying x"
//  */
// ```

// <!-- tabs:end -->

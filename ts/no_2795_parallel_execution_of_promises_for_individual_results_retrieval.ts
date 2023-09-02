// # [2795. Parallel Execution of Promises for Individual Results Retrieval](https://leetcode.com/problems/parallel-execution-of-promises-for-individual-results-retrieval)

// ## Description

// Given an array functions, return a promise promise. 
// functions is an array of functions that return promises fn Promise. 
// Each fn Promise can be resolved or rejected.  

// If fn Promise is resolved:

//     obj = { status: "fulfilled", value: resolved value}

// If fn Promise is rejected:

//     obj = { status: "rejected", reason: reason of rejection (catched error message)}

// The promise should resolve with an array of these objects obj. 
// Each obj in the array should correspond to the promises in the original array function,
//  maintaining the same order.

// Try to implement it without using the built-in method Promise.allSettled().

 
//  ### Example 1:


// Input: functions = [
//     () =>  new Promise(resolve =>  setTimeout(() =>  resolve(15), 100))
// ]
// Output: {"t":100,"values":[{"status":"fulfilled","value":15}]}
// Explanation: 
// const time = performance.now()
// const promise = promiseAllSettled(functions);
               
// promise.then(res =>  {
//     const out = {t: Math.floor(performance.now() - time), values: res}
//     console.log(out) // {"t":100,"values":[{"status":"fulfilled","value":15}]}
// })

// The returned promise resolves within 100 milliseconds. 
// Since promise from the array functions is fulfilled, 
// the resolved value of the returned promise is set to [{"status":"fulfilled","value":15}].


//  ### Example 2:


// Input: functions = [
//     () =>  new Promise(resolve =>  setTimeout(() =>  resolve(20), 100)), 
//     () =>  new Promise(resolve =>  setTimeout(() =>  resolve(15), 100))
// ]
// Output: 
// {
//     "t":100,
//     "values": [
//         {"status":"fulfilled","value":20},
//         {"status":"fulfilled","value":15}
//     ]
// }
// Explanation: The returned promise resolves within 100 milliseconds,
//  because the resolution time is determined by the promise that takes the longest time to fulfill. 
// Since promises from the array functions are fulfilled, 
// the resolved value of the returned promise is set to [{"status":"fulfilled","value":20},{"status":"fulfilled","value":15}].


//  ### Example 3:


// Input: functions = [
//     () =>  new Promise(resolve =>  setTimeout(() =>  resolve(30), 200)), 
//     () =>  new Promise((resolve, reject) =>  setTimeout(() =>  reject("Error"), 100))
// ]
// Output:
// {
//     "t":200,
//     "values": [
//         {"status":"fulfilled","value":30},
//         {"status":"rejected","reason":"Error"}
//     ]
// }
// Explanation: The returned promise resolves within 200 milliseconds, 
// as its resolution time is determined by the promise that takes the longest time to fulfill. 
// Since one promise from the array function is fulfilled and another is rejected, 
// the resolved value of the returned promise is set to an array containing objects in the following order: 
// [{"status":"fulfilled","value":30}, {"status":"rejected","reason":"Error"}]. 
// Each object in the array corresponds to the promises in the original array function, maintaining the same order.


 
// Constraints:


// 	1 <= functions.length <= 10


// ## Solutions

// <!-- tabs:start -->

// ### **TypeScript**

// ```ts
// type FulfilledObj = {
//     status: 'fulfilled';
//     value: string;
// };
// type RejectedObj = {
//     status: 'rejected';
//     reason: string;
// };
// type Obj = FulfilledObj | RejectedObj;

// function promiseAllSettled(functions: Function[]): Promise<Obj[]> {
//     return new Promise(resolve => {
//         const res: Obj[] = [];
//         let count = 0;
//         for (let i in functions) {
//             functions[i]()
//                 .then(value => ({ status: 'fulfilled', value }))
//                 .catch(reason => ({ status: 'rejected', reason }))
//                 .then(obj => {
//                     res[i] = obj;
//                     if (++count === functions.length) {
//                         resolve(res);
//                     }
//                 });
//         }
//     });
// }

// /**
//  * const functions = [
//  *    () => new Promise(resolve => setTimeout(() => resolve(15), 100))
//  * ]
//  * const time = performance.now()
//  *
//  * const promise = promiseAllSettled(functions);
//  *
//  * promise.then(res => {
//  *     const out = {t: Math.floor(performance.now() - time), values: res}
//  *     console.log(out) // {"t":100,"values":[{"status":"fulfilled","value":15}]}
//  * })
//  */
// ```

// ### **JavaScript**

// ```js
// /**
//  * @param {Array} functions
//  * @return {Promise}
//  */
// var promiseAllSettled = function (functions) {
//     return new Promise(resolve => {
//         const res = [];
//         let count = 0;
//         for (let i in functions) {
//             functions[i]()
//                 .then(value => ({ status: 'fulfilled', value }))
//                 .catch(reason => ({ status: 'rejected', reason }))
//                 .then(obj => {
//                     res[i] = obj;
//                     if (++count === functions.length) {
//                         resolve(res);
//                     }
//                 });
//         }
//     });
// };
// ```

// <!-- tabs:end -->

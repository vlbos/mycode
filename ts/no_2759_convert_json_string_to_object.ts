// # [2759. Convert JSON String to Object](https://leetcode.com/problems/convert-json-string-to-object)


// ## Description

// Given a string str, return parsed JSON parsedStr. 
// You may assume the str is a valid JSON string hence it only includes strings, numbers, arrays, objects, booleans, 
// and null. str will not include invisible characters and escape characters. 
// String values within the JSON will only contain alphanumeric characters.

// Please solve it without using the built-in JSON.parse method.

 
// ### Example 1:


// Input: str = '{"a":2,"b":[1,2,3]}'
// Output: {"a":2,"b":[1,2,3]}
// Explanation: Returns the object represented by the JSON string.

// ### Example 2:


// Input: str = 'true'
// Output: true
// Explanation: Primitive types are valid JSON.

// ### Example 3:


// Input: str = '[1,5,"false",{"a":2}]'
// Output: [1,5,"false",{"a":2}]
// Explanation: Returns the array represented by the JSON string.

 
// Constraints:


// 	str is a valid JSON string
// 	1  <= str.length  <= 105


// ## Solutions

// <!-- tabs:start -->

// ### **TypeScript**

// ```ts
// function jsonParse(str: string): any {
//     const n = str.length;
//     let i = 0;

//     const parseTrue = (): boolean => {
//         i += 4;
//         return true;
//     };

//     const parseFalse = (): boolean => {
//         i += 5;
//         return false;
//     };

//     const parseNull = (): null => {
//         i += 4;
//         return null;
//     };

//     const parseNumber = (): number => {
//         let s = '';
//         while (i < n) {
//             const c = str[i];
//             if (c === ',' || c === '}' || c === ']') {
//                 break;
//             }
//             s += c;
//             i++;
//         }
//         return Number(s);
//     };

//     const parseArray = (): any[] => {
//         const arr: any[] = [];
//         i++;
//         while (i < n) {
//             const c = str[i];
//             if (c === ']') {
//                 i++;
//                 break;
//             }
//             if (c === ',') {
//                 i++;
//                 continue;
//             }
//             const value = parseValue();
//             arr.push(value);
//         }
//         return arr;
//     };

//     const parseString = (): string => {
//         let s = '';
//         i++;
//         while (i < n) {
//             const c = str[i];
//             if (c === '"') {
//                 i++;
//                 break;
//             }
//             if (c === '\\') {
//                 i++;
//                 s += str[i];
//             } else {
//                 s += c;
//             }
//             i++;
//         }
//         return s;
//     };

//     const parseObject = (): any => {
//         const obj: any = {};
//         i++;
//         while (i < n) {
//             const c = str[i];
//             if (c === '}') {
//                 i++;
//                 break;
//             }
//             if (c === ',') {
//                 i++;
//                 continue;
//             }
//             const key = parseString();
//             i++;
//             const value = parseValue();
//             obj[key] = value;
//         }
//         return obj;
//     };
//     const parseValue = (): any => {
//         const c = str[i];
//         if (c === '{') {
//             return parseObject();
//         }
//         if (c === '[') {
//             return parseArray();
//         }
//         if (c === '"') {
//             return parseString();
//         }
//         if (c === 't') {
//             return parseTrue();
//         }
//         if (c === 'f') {
//             return parseFalse();
//         }
//         if (c === 'n') {
//             return parseNull();
//         }
//         return parseNumber();
//     };
//     return parseValue();
// }
// ```

// <!-- tabs:end -->

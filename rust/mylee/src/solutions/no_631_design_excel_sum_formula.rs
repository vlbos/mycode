// 631\. Design Excel Sum Formula
// ==============================

// Your task is to design the basic function of Excel and implement the function of sum formula. Specifically, you need to implement the following functions:

// `Excel(int H, char W):` This is the constructor. The inputs represents the height and width of the Excel form. **H** is a positive integer, range from 1 to 26.
// It represents the height. **W** is a character range from 'A' to 'Z'. It represents that the width is the number of characters from 'A' to **W**.
//  The Excel form content is represented by a height \* width 2D integer array `C`, it should be initialized to zero.
// You should assume that the first row of `C` starts from 1, and the first column of `C` starts from 'A'.

// `void Set(int row, char column, int val):` Change the value at `C(row, column)` to be val.

// `int Get(int row, char column):` Return the value at `C(row, column)`.

// `int Sum(int row, char column, List of Strings : numbers):` This function calculate and set the value at `C(row, column)`,
// where the value should be the sum of cells represented by `numbers`. This function return the sum result at `C(row, column)`.
// This sum formula should exist until this cell is overlapped by another value or another sum formula.

// `numbers` is a list of strings that each string represent a cell or a range of cells. If the string represent a single cell,
// then it has the following format : `ColRow`. For example, "F7" represents the cell at (7, F).

// If the string represent a range of cells, then it has the following format : `ColRow1:ColRow2`. The range will always be a rectangle,
// and ColRow1 represent the position of the top-left cell, and ColRow2 represents the position of the bottom-right cell.

// **Example 1:**

// Excel(3,"C");
// // construct a 3\*3 2D array with all zero.
// //   A B C
// // 1 0 0 0
// // 2 0 0 0
// // 3 0 0 0

// Set(1, "A", 2);
// // set C(1,"A") to be 2.
// //   A B C
// // 1 2 0 0
// // 2 0 0 0
// // 3 0 0 0

// Sum(3, "C", \["A1", "A1:B2"\]);
// // set C(3,"C") to be the sum of value at C(1,"A") and the values sum of the rectangle range whose top-left cell is C(1,"A") and bottom-right cell is C(2,"B"). Return 4.
// //   A B C
// // 1 2 0 0
// // 2 0 0 0
// // 3 0 0 4

// Set(2, "B", 2);
// // set C(2,"B") to be 2. Note C(3, "C") should also be changed.
// //   A B C
// // 1 2 0 0
// // 2 0 2 0
// // 3 0 0 6

// **Note:**

// 1.  You could assume that there won't be any circular sum reference. For example, A1 = sum(B1) and B1 = sum(A1).
// 2.  The test cases are using double-quotes to represent a character.
// 3.  Please remember to **RESET** your class variables declared in class Excel, as static/class variables are **persisted across multiple test cases**. Please see [here](https://leetcode.com/faq/#different-output) for more details.

// ### Difficulty:

// Hard

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [Microsoft](https://leetcode.ca/tags/#Microsoft)

// @lc code=start

// const A_CHAR_CODE: usize = 'A' as u8 as usize;

// #[derive(Clone)]
// struct ExcelNode {
//     value: i32,
//     watches: Vec<Vec<(usize, usize)>>,
//     observers: HashMap<(usize, usize), usize>,
// }

// impl ExcelNode {
//     pub fn new(value: i32) -> Self {
//         Self {
//             value,
//             watches: vec![],
//             observers: HashMap::new(),
//         }
//     }

//     pub fn watches_to_vec(&self) -> Vec<(usize, usize)> {
//         let mut res = vec![];
//         for s in &self.watches {
//             if s.len() == 1 {
//                 res.push(s[0]);
//             } else {
//                 let (fr, fc) = s[0];
//                 let (tr, tc) = s[1];
//                 for r in fr..=tr {
//                     for c in fc..=tc {
//                         res.push((r, c));
//                     }
//                 }
//             }
//         }
//         res
//     }

//     pub fn observers_to_vec(&self) -> Vec<(usize, usize)> {
//         let mut res = vec![];
//         for (&k, &v) in &self.observers {
//             for _ in 0..v {
//                 res.push(k);
//             }
//         }
//         res
//     }
// }
use std::collections::HashMap;

struct Excel {
    // values: Vec<Vec<ExcelNode>>,
    // rows: usize,
    // cols: usize,
    mat: Vec<Vec<i32>>,
    m: HashMap<(i32, char), Vec<String>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Excel {
    pub fn new(h: i32, w: char) -> Self {
        // let cols = h as usize;
        // let rows = Excel::col_c2i(w as u8) + 1;
        // Self {
        //     values: vec![vec![ExcelNode::new(0); cols]; rows],
        //     rows,
        //     cols,
        // }
        Self {
            mat: vec![vec![0; h as usize]; (w as u8 - b'A') as usize + 1],
            m: HashMap::new(),
        }
    }

    pub fn set(&mut self, r: i32, c: char, v: i32) {
        // let col = Excel::col_c2i(c as u8);
        // let row = Excel::row_i2i(r);
        // self.set_impl(row, col, v);
        // self.clear_watches(row, col);
        if self.m.contains_key(&(r, c)) {
            self.m.remove(&(r, c));
        }
        self.mat[r as usize - 1][(c as u8 - b'A') as usize] = v;
    }

    pub fn get(&mut self, r: i32, c: char) -> i32 {
        // let col = Excel::col_c2i(c as u8);
        // let row = Excel::row_i2i(r);
        // self.retrieve(row, col).value
        if let Some(strs) = self.m.get(&(r, c)) {
            return self.sum(r, c, strs.clone());
        }
        self.mat[r as usize - 1][(c as u8 - b'A') as usize]
    }

    pub fn sum(&mut self, r: i32, c: char, strs: Vec<String>) -> i32 {
        // let col = Excel::col_c2i(c as u8);
        // let row = Excel::row_i2i(r);
        // self.set_impl(row, col, 0);
        // self.clear_watches(row, col);
        // self.add_watches(row, col, strs);
        // self.retrieve(row, col).value
        let mut ans = 0;
        for str in &strs {
            if let Some(k) = str.rfind(":") {
                let (x1, y1) = (
                    str[1..k].parse::<usize>().unwrap(),
                    (str.as_bytes()[0] - b'A') as usize,
                );
                let (x2, y2) = (
                    str[k + 2..].parse::<usize>().unwrap(),
                    (str.as_bytes()[k + 1] - b'A') as usize,
                );
                for i in x1..=x2 {
                    for j in y1..=y2 {
                        ans += self.get(i as i32, (b'A' + j as u8) as char);
                    }
                }
            } else {
                ans += self.get(
                    str[1..].parse::<i32>().unwrap(),
                    str.chars().nth(0).unwrap(),
                );
            }
        }
        self.m.insert((r, c), strs);
        ans
    }

    // fn set_impl(&mut self, row: usize, col: usize, v: i32) {
    //     let sub_v = v - self.retrieve(row, col).value;
    //     let mut stack = vec![(row, col)];
    //     while let Some((r, c)) = stack.pop() {
    //         let mut node = self.retrieve_mut(r, c);
    //         node.value += sub_v;
    //         stack.extend(node.observers_to_vec().into_iter());
    //     }
    // }

    // fn retrieve_mut(&mut self, r: usize, c: usize) -> &mut ExcelNode {
    //     &mut self.values[r][c]
    // }

    // fn retrieve(&self, r: usize, c: usize) -> &ExcelNode {
    //     &self.values[r][c]
    // }

    // fn clear_watches(&mut self, row: usize, col: usize) {
    //     let node = self.retrieve_mut(row, col);
    //     let watches = node.watches_to_vec();
    //     node.watches = vec![];

    //     let node_index = (row, col);
    //     for (r, c) in watches {
    //         let another = self.retrieve_mut(r, c);
    //         if another.observers[&node_index] == 1 {
    //             another.observers.remove(&node_index);
    //         } else {
    //             *another.observers.get_mut(&node_index).unwrap() -= 1;
    //         }
    //     }
    // }

    // fn add_watches(&mut self, row: usize, col: usize, strs: Vec<String>) {
    //     let node = self.retrieve_mut(row, col);
    //     for s in strs {
    //         let splits = s.split(":").collect::<Vec<_>>();
    //         if splits.len() == 1 {
    //             let sp = Excel::colrow_to_index(&splits[0]);
    //             node.watches.push(vec![sp]);
    //         } else {
    //             let fp = Excel::colrow_to_index(&splits[0]);
    //             let tp = Excel::colrow_to_index(&splits[1]);
    //             node.watches.push(vec![fp, tp]);
    //         }
    //     }
    //     let watches = node.watches_to_vec();
    //     let node_index = (row, col);
    //     let mut sum = 0;
    //     for (r, c) in watches {
    //         let another = self.retrieve_mut(r, c);
    //         another
    //             .observers
    //             .entry(node_index)
    //             .and_modify(|v| *v += 1)
    //             .or_insert(1);
    //         sum += another.value;
    //     }
    //     self.set_impl(row, col, sum);
    // }

    // #[inline]
    // fn col_c2i(c: u8) -> usize {
    //     c as usize - A_CHAR_CODE
    // }

    // #[inline]
    // fn row_s2i(c: &str) -> usize {
    //     c.parse::<usize>().unwrap() - 1
    // }

    // #[inline]
    // fn row_i2i(i: i32) -> usize {
    //     (i - 1) as usize
    // }

    // #[inline]
    // fn colrow_to_index(colrow: &str) -> (usize, usize) {
    //     let col = Excel::col_c2i(colrow.as_bytes()[0]);
    //     let row = Excel::row_s2i(&colrow[1..]);
    //     (row, col)
    // }

    // fn to_matrix(&self) -> Vec<Vec<i32>> {
    //     self.values
    //         .iter()
    //         .map(|v| v.iter().map(|n| n.value).collect::<Vec<i32>>())
    //         .collect::<Vec<_>>()
    // }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    use crate::{lc_matrix, lc_vec_s};

    #[test]
    fn test_excel_1() {
        let mut excel = Excel::new(3, 'C');
        assert_eq!(excel.mat, lc_matrix![[0, 0, 0], [0, 0, 0], [0, 0, 0]]);
        excel.set(1, 'A', 2);
        assert_eq!(excel.mat, lc_matrix![[2, 0, 0], [0, 0, 0], [0, 0, 0]]);
        assert_eq!(excel.sum(3, 'C', lc_vec_s!["A1", "A1:B2"]), 4);
        assert_eq!(excel.mat, lc_matrix![[2, 0, 0], [0, 0, 0], [0, 0, 0]]);
        excel.set(2, 'B', 2);
        assert_eq!(excel.mat, lc_matrix![[2, 0, 0], [0, 2, 0], [0, 0, 0]]);
    }

    #[test]
    fn test_excel_2() {
        let mut excel = Excel::new(5, 'E');
        assert_eq!(excel.get(1, 'A'), 0);
        excel.set(1, 'A', 1);
        assert_eq!(excel.get(1, 'A'), 1);
        assert_eq!(excel.sum(2, 'B', lc_vec_s!["A1", "A1"]), 2);
        excel.set(1, 'A', 2);
        assert_eq!(excel.get(2, 'B'), 4);
    }
}

// 604\. Design Compressed String Iterator
// =======================================

// Design and implement a data structure for a compressed string iterator. It should support the following operations: `next` and `hasNext`.

// The given compressed string will be in the form of each letter followed by a positive integer representing the number of this letter existing in the original uncompressed string.

// `next()` \- if the original string still has uncompressed characters, return the next letter; Otherwise return a white space.
// `hasNext()` \- Judge whether there is any letter needs to be uncompressed.

// **Note:**
// Please remember to **RESET** your class variables declared in StringIterator, as static/class variables are **persisted across multiple test cases**.
// Please see [here](https://leetcode.com/faq/#different-output) for more details.

// **Example:**

// StringIterator iterator = new StringIterator("L1e2t1C1o1d1e1");

// iterator.next(); // return 'L'
// iterator.next(); // return 'e'
// iterator.next(); // return 'e'
// iterator.next(); // return 't'
// iterator.next(); // return 'C'
// iterator.next(); // return 'o'
// iterator.next(); // return 'd'
// iterator.hasNext(); // return true
// iterator.next(); // return 'e'
// iterator.hasNext(); // return false
// iterator.next(); // return ' '

// ### Difficulty:

// Easy

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [Google](https://leetcode.ca/tags/#Google)

// @lc code=start

// #[derive(Clone, Debug)]
// enum StringIteratorState {
//     Unknown,
//     Current { content: char, repeat: usize },
//     Empty,
// }

#[derive(Clone, Debug)]
pub  struct StringIterator {
    // source: Vec<char>,
    // cursor: usize,
    // state: StringIteratorState,
    s: Vec<char>,
    t: Vec<i32>,
    idx: usize,
    i: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StringIterator {
    pub fn new(compressed_string: String) -> Self {
        // let mut source = compressed_string.chars().collect::<Vec<_>>();
        // source.push('#');
        // Self {
        //     source,
        //     cursor: 0,
        //     state: StringIteratorState::Unknown,
        // }
        let s: Vec<char> = compressed_string
            .split(char::is_numeric)
            .filter(|x| !x.is_empty())
            .map(|x| x.chars().nth(0).unwrap_or(' '))
            .collect();
        let t: Vec<i32> = compressed_string
            .split(char::is_alphabetic)
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<i32>().unwrap_or(0))
            .collect();
        println!("{:?},{:?}", &s, &t);
        Self { s, t, idx: 0, i: 0 }
    }

    // pub fn consume_next(&mut self) {
    //     // if let StringIteratorState::Unknown = self.state {
    //     //     let mut i = self.cursor;
    //     //     let ch = self.source[i];
    //     //     self.state = if ch == '#' {
    //     //         StringIteratorState::Empty
    //     //     } else {
    //     //         let content = ch;
    //     //         let mut num_str = String::new();
    //     //         while i < self.source.len() {
    //     //             i += 1;
    //     //             let c = self.source[i];
    //     //             if c.is_digit(10) {
    //     //                 num_str.push(c);
    //     //             } else {
    //     //                 break;
    //     //             }
    //     //         }
    //     //         self.cursor = i;
    //     //         StringIteratorState::Current {
    //     //             content,
    //     //             repeat: num_str.parse::<usize>().unwrap(),
    //     //         }
    //     //     };
    //     // }
    // }

    pub fn next(&mut self) -> char {
        // self.consume_next();
        // let res = match self.state {
        //     StringIteratorState::Empty => ' ',
        //     StringIteratorState::Current { content, repeat: _ } => content,
        //     _ => unreachable!(),
        // };
        // if let StringIteratorState::Current { content, repeat } = self.state {
        //     self.state = if repeat == 1 {
        //         StringIteratorState::Unknown
        //     } else {
        //         StringIteratorState::Current {
        //             content,
        //             repeat: repeat - 1,
        //         }
        //     };
        // };
        // res
        if self.idx == self.s.len() {
            return ' ';
        }
        if self.i < self.t[self.idx] {
            self.i += 1;
        } else {
            self.idx += 1;
            if self.idx == self.s.len() {
                return ' ';
            }
            self.i = 1;
        }
        self.s[self.idx]
    }

    pub fn has_next(&mut self) -> bool {
        // self.consume_next();
        // match self.state {
        //     StringIteratorState::Empty => false,
        //     StringIteratorState::Current {
        //         content: _,
        //         repeat: _,
        //     } => true,
        //     _ => unreachable!(),
        // }
        self.idx + 1 < self.s.len() || (self.idx + 1 == self.s.len() && self.i < self.t[self.idx])
    }
}

// @lc code=end

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_string_iterator_1() {
        let mut iter = StringIterator::new(String::from("L1e2t1C1o1d1e1"));
        let res = vec!['L', 'e', 'e', 't', 'C', 'o', 'd'];
        for r in res {
            assert_eq!(iter.next(), r);
        }
        assert_eq!(iter.has_next(), true);
        assert_eq!(iter.next(), 'e');
        assert_eq!(iter.has_next(), false);
        assert_eq!(iter.next(), ' ');
    }
}

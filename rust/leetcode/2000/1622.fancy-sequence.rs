/*
 * @lc app=leetcode id=1622 lang=rust
 *
 * [1622] Fancy Sequence
 */

// @lc code=start
struct Fancy {
    v: Vec<i32>,
    a: i32,
    b: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
const p: i64 = 1_000_000_007;
impl Fancy {
    fn new() -> Self {
        Self {
            v: Vec::new(),
            a: 1,
            b: 0,
        }
    }

    fn append(&mut self, val: i32) {
        let quick_mul = |x: i32, mut y: i64| {
            let mut ans = 1;
            let mut cur = x;
            while y > 0 {
                if y & 1 > 0 {
                    ans = ((ans as i64) * (cur as i64) % p) as i32;
                }
                cur = ((cur as i64) * (cur as i64) % p) as i32;
                y >>= 1;
            }
            ans
        };
        let inv = |x: i32| quick_mul(x, p - 2) as i64;
        self.v
            .push(((((val - self.b) as i64) + p) % p * inv(self.a) % p) as i32);
    }

    fn add_all(&mut self, inc: i32) {
        self.b = (((self.b as i64)+ inc as i64)  % p) as i32;
    }

    fn mult_all(&mut self, m: i32) {
        self.a = ((self.a as i64) * (m as i64) % p) as i32;
        self.b = ((self.b as i64) * (m as i64) % p) as i32;
    }

    fn get_index(&self, idx: i32) -> i32 {
        if idx >= self.v.len() as i32 {
            return -1;
        }
        ((((self.a as i64) * (self.v[idx as usize] as i64)) % p + self.b as i64) % p) as i32
    }
}
/**
 * Your Fancy object will be instantiated and called as such:
 * let obj = Fancy::new();
 * obj.append(val);
 * obj.add_all(inc);
 * obj.mult_all(m);
 * let ret_4: i32 = obj.get_index(idx);
 */
// @lc code=end

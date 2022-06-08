/*
 * @lc app=leetcode id=2166 lang=rust
 *
 * [2166] Design Bitset
 */

// @lc code=start
struct Bitset {
    cnt: usize,
    bits: Vec<u8>,
    reversed: u8,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Bitset {
    fn new(size: i32) -> Self {
        Self {
            cnt: 0,
            bits: vec![0u8; size as usize],
            reversed: 0,
        }
    }

    fn fix(&mut self, idx: i32) {
        let i = idx as usize;
        if self.bits[i] ^ self.reversed == 0 {
            self.bits[i] ^= 1;
            self.cnt += 1;
        }
    }

    fn unfix(&mut self, idx: i32) {
        let i = idx as usize;
        if self.bits[i] ^ self.reversed == 1 {
            self.bits[i] ^= 1;
            self.cnt -= 1;
        }
    }

    fn flip(&mut self) {
        self.cnt = self.bits.len() - self.cnt;
        self.reversed ^= 1;
    }

    fn all(&self) -> bool {
        self.bits.len() == self.cnt
    }

    fn one(&self) -> bool {
        self.cnt > 0
    }

    fn count(&self) -> i32 {
        self.cnt as _
    }

    fn to_string(&self) -> String {
        String::from_utf8(
            self.bits
                .iter()
                .cloned()
                .map(|x| (x ^ self.reversed) + b'0')
                .collect::<Vec<u8>>(),
        )
        .unwrap()
    }
}

/**
 * Your Bitset object will be instantiated and called as such:
 * let obj = Bitset::new(size);
 * obj.fix(idx);
 * obj.unfix(idx);
 * obj.flip();
 * let ret_4: bool = obj.all();
 * let ret_5: bool = obj.one();
 * let ret_6: i32 = obj.count();
 * let ret_7: String = obj.to_string();
 */
// @lc code=end

理解常数时间（Constant-time）是编写加密代码与普通代码最大的区别。为了防止攻击者通过测量 CPU 执行时间来推算你的密钥，必须消除所有与秘密数据相关的分支预测。
以下是实现“判断并减去模数”逻辑的 Rust 常数时间写法示例：
1. 核心逻辑：条件减法 (Conditional Subtraction)
假设我们要实现 if x >= P { x - P } else { x }，但不能用 if。
rust
/// 常数时间：如果 x >= P，则返回 x - P，否则返回 x
fn ct_reduce_once(x: u64, p: u64) -> u64 {
    // 1. 计算差值（可能发生借位/溢出）
    let (diff, borrow) = x.overflowing_sub(p);
    
    // 2. 生成掩码：
    // 如果发生借位（x < p），borrow 为 1，mask 为 0x0000... (全0)
    // 如果未发生借位（x >= p），borrow 为 0，mask 为 0xFFFF... (全1)
    let mask = (borrow as u64).wrapping_sub(1);
    
    // 3. 选择结果：
    // 如果 mask 为全1，保留 diff (即 x - p)
    // 如果 mask 为全0，保留 x
    (diff & mask) | (x & !mask)
}
请谨慎使用此类代码。

2. 在 
 基数下的进位处理 (Carry Propagation)
当你手动实现 fe25519 的加法后，需要处理每个肢体（limb）的进位，同样要保持常数时间：
rust
impl Fe25519 {
    pub fn carry_propagate(&mut self) {
        let mask = (1u64 << 51) - 1;
        
        // 依次处理 5 个 limb 的进位
        for i in 0..4 {
            let carry = self.0[i] >> 51;
            self.0[i] &= mask;
            self.0[i + 1] += carry;
        }
        
        // 最后一项的处理：Ed25519 的模数是 2^255 - 19
        // 超过 2^255 的部分需要乘以 19 并加回到最低位
        let final_carry = self.0[4] >> 51;
        self.0[4] &= mask;
        self.0[0] += final_carry * 19;
        
        // 再次进位以处理由于乘以 19 可能产生的微量溢出
        // (在高性能实现中，通常会延迟到最后一步才做彻底约简)
    }
}
请谨慎使用此类代码。

3. Rust 加密开发的“黄金法则”
不要使用 f32/f64：浮点运算的执行时间在不同 CPU 上极其不稳定。
不要使用数组索引作为秘密值：例如 array[secret_index] 会导致缓存侧信道攻击。
使用 subtle crate：在生产环境中，通常会使用 subtle 这个库，它提供了 Choice、ConditionallySelectable 等 Trait，帮你安全地实现这些位运算逻辑。
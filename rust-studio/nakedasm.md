/// SAFETY: 遵循 64 位 System-V ABI 约定。
#[unsafe(naked)]
pub extern "sysv64" fn wrapping_add(a: u64, b: u64) -> u64 {
    // 相当于 `a.wrapping_add(b)`。
    core::arch::naked_asm!(
        "lea rax, [rdi + rsi]",
        "ret"
    );
}
为什么要用裸函数？
等等，如果裸函数只是 global_asm! 的语法糖，为什么还要引入它？

让我们通过一个例子来说明，用 global_asm! 重写上面的 wrapping_add 函数会变成这样：

// SAFETY: `wrapping_add` 是在此模块中定义的，使用 64 位 System-V ABI。
unsafe extern "sysv64" {
    fn wrapping_add(a: u64, b: u64) -> u64;
}

core::arch::global_asm!(
    r#"
        // 用于设置函数的指令（平台相关）
        .section .text.wrapping_add,"ax",@progbits
        .p2align 2
        .globl wrapping_add
        .type wrapping_add,@function

wrapping_add:
        lea rax, [rdi + rsi]
        ret

.Ltmp0:
        .size wrapping_add, .Ltmp0-wrapping_add
    "#
);
使用 naked 更方便的几个理由：
自动生成平台相关指令
使用 global_asm! 时你必须自己写 .section、.type 等平台特有的指令，而裸函数会自动生成这些内容。
名字参与 Rust 的名称修饰（mangling）
global_asm! 中硬编码的函数名不会参与 Rust 的名字修饰机制，这会导致跨平台开发困难。裸函数则遵循 Rust 的名字修饰规则，避免符号冲突。
支持泛型
用 global_asm! 定义的函数不能使用泛型（包括 const 泛型），而裸函数可以。
统一的注释与文档位置
裸函数只有一个定义位置，因此注释、文档和属性（如 #[doc]、#[inline]）不容易漏掉或过时，安全注释可以紧跟函数体，非常重要。
裸函数之所以是 unsafe 的，是因为函数签名、ABI 和实现必须严格匹配，一旦不一致可能造成未定义行为。
发展历程
裸函数这个功能，其实早在 2015 年就提出过 RFC。
但后来在 2020 年被 RFC 2972 取代。当时 Rust 的内联汇编语法已有重大改进，因此新的 RFC 限制裸函数的函数体必须是单个 asm! 调用，并加入了一些限制条件。

现在，距离最初提议已经 10 年，裸函数终于稳定了。

在这个过程中，有两个关键改进为稳定打下了基础：

裸函数背后的技术实现
引入 naked_asm! 宏
裸函数的函数体必须是一个 naked_asm! 宏。这是介于 asm! 和 global_asm! 之间的一种新形式：

像 asm! 一样，它在函数体中使用；
但像 global_asm! 一样，它只接受部分操作数类型。
原本的实现尝试在 asm! 上加 lint，但难以精确定义行为和报错提示。而 naked_asm! 作为专门为裸函数设计的宏，更容易规范和理解。



extern "custom" 调用约定
虽然裸函数通常使用 extern "C"，但这在很多情况下是假的——它们用的是自定义 ABI。

为此，Rust 引入了 abi_custom 特性，支持 extern "custom"。比如 Rust 的 compiler-builtins 中就有如下用法：

#![feature(abi_custom)]

/// 使用 ARM 的非标准 ABI 实现除法和取模
/// ```c
/// typedef struct { int quot; int rem; } idiv_return;
///  __value_in_regs idiv_return __aeabi_idivmod(int num, int denom);
/// ```
/// SAFETY: 汇编实现了预期的 ABI，且 "custom" 避免被 Rust 代码直接调用
#[unsafe(naked)]
pub unsafe extern "custom" fn __aeabi_idivmod() {
    core::arch::naked_asm!(
        "push {{r0, r1, r4, lr}}", // 保存寄存器
        "bl {trampoline}",         // 调用除法函数
        "pop {{r1, r2}}",
        "muls r2, r2, r0",         // 求余
        "subs r1, r1, r2",
        "pop {{r4, pc}}",          // 恢复寄存器并返回（设置 pc）
        trampoline = sym crate::arm::__aeabi_idiv,
    );
}
由于 Rust 编译器不理解 custom 的调用约定，所以不能通过常规函数调用去调用它，只能用汇编间接执行。
在汇编行上使用 #[cfg(...)]
新特性 cfg_asm 允许给每一行汇编加条件编译，非常适合做平台适配。例如：

#![feature(cfg_asm)]

global_asm!(
    // ...
    // 如果启用，则设置栈指针
    #[cfg(feature = "set-sp")]
    "ldr r0, =_stack_start
     msr msp, r0",
    // ...
);
这个例子来自 cortex-m 库，之前他们为了条件编译，必须复制整段汇编块，而现在只需要加一行 #[cfg] 就行。


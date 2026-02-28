#![no_std]
#![no_main]

#[macro_use]
mod uart;

use core::arch::global_asm;
use core::panic::PanicInfo;



// 1. 内核第一条指令的汇编入口
// OpenSBI 会跳转到这里，我们需要先设置栈指针 (sp)，否则 Rust 函数无法调用喵
global_asm!(
    ".section .text.entry",
    ".globl _start",
    "_start:",
    "    la sp, boot_stack_top",     // 加载栈顶地址到 sp 寄存器
    "    call rust_main",            // 跳转到 Rust 的初始化逻辑
    "1:",
    "    wfi",                       // (Wait For Interrupt) 睡眠等待
    "    j 1b",                      // 死循环，防止内核退出
    ".section .bss.uninit",
    ".space 4096 * 16",              // 预留 64KB 的内存作为启动栈
    "boot_stack_top:"                // 栈顶标签（栈在 RISC-V 中向下增长）
);

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    clear_bss();
    
    // --- 新增：内核的第一声啼哭喵！ ---
    println!("===============================");
    println!(" Nyako OS (Linux 0.11 on RISC-V) ");
    println!(" Kernel initialized successfully!");
    println!("===============================");
    
    loop {}
}

// 3. 内存清理机制
// 在裸机环境下，加载器不会帮我们清空 BSS 段，如果包含历史垃圾数据会导致严重 Bug 喵
fn clear_bss() {
    unsafe extern "C" {
        fn sbss(); // 符号由 linker.ld 注入
        fn ebss();
    }
    unsafe {
        core::slice::from_raw_parts_mut(
            sbss as *const () as usize as *mut u8,
            ebss as *const () as usize - sbss as *const () as usize,
        )
        .fill(0);
    }
}

// 4. 崩溃处理钩子
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
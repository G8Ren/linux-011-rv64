use core::fmt::{self, Write};

// QEMU virt 机器中 UART 芯片的 MMIO 物理基地址
const UART0: usize = 0x1000_0000;

pub struct Uart;

impl Uart {
    // 往串口的数据寄存器（偏移为0）写入一个字节
    pub fn put_char(c: u8) {
        let ptr = UART0 as *mut u8;
        unsafe {
            // write_volatile 保证编译器不会因为“无意义的内存写入”而优化掉这行代码喵
            ptr.write_volatile(c);
        }
    }
}

// 实现 Rust 核心库的格式化写入 trait
impl fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            Uart::put_char(byte);
        }
        Ok(())
    }
}

// 供宏调用的底层打印函数
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    Uart.write_fmt(args).unwrap();
}

// 导出我们自定义的宏，屏蔽底层调用细节喵
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::uart::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
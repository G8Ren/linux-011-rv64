OUTPUT_ARCH(riscv)
ENTRY(_start)
BASE_ADDRESS = 0x80200000;

SECTIONS
{
    . = BASE_ADDRESS;
    
    /* 强行将汇编的 _start 符号所在的段放到整个二进制文件的最开头 */
    .text : {
        *(.text.entry)
        *(.text .text.*)
    }
    
    .rodata : {
        *(.rodata .rodata.*)
    }
    
    .data : {
        *(.data .data.*)
    }
    
    .bss : {
        *(.bss.uninit)
        . = ALIGN(4K);
        PROVIDE(sbss = .);
        *(.bss .bss.*)
        PROVIDE(ebss = .);
    }
    
    /DISCARD/ : {
        *(.eh_frame)
    }
}
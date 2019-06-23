/*
    bios: basic input/output system
    bios boot: bios -> bootable disk -> bootloader (512kb executable) -> kernel image
    vga buffer:25rows, 80columns
*/
#![no_std] //不使用设计系统调用的标准库
#![no_main] //关闭Rust-Level的入口
#![feature(asm)]

mod vga_buffer;

use core::panic::PanicInfo;
//use core::fmt::Write;

#[panic_handler] //当Panic时调用，因为原本的panic是rust标准库的一部分
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle] //不允许更改函数名
pub extern "C" fn _start() -> ! {
    //此函数是入口，linker会默认寻找一个叫做 "_start" 的函数作为入口
    //    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    println!("Hello World{}", "!");
    panic!("Some panic message");
    loop {}
}

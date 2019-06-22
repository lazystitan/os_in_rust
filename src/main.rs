/*
    bios: basic input/output system
    bios boot: bios -> bootable disk -> bootloader (512kb executable) -> kernel image
*/
#![no_std] //不使用设计系统调用的标准库
#![no_main]//关闭Rust-Level的入口
#![feature(asm)]

use core::panic::PanicInfo;

#[panic_handler]//当Panic时调用，因为原本的panic是rust标准库的一部分
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]//不允许更改函数名
pub extern "C" fn _start() -> ! {
    //此函数是入口，linker会默认寻找一个叫做 "_start" 的函数作为入口
    let vga_buffer = 0xb8000 as *mut u8;
    //0xb8000是vga buffer的地址

    for (i, &byte) in HELLO.iter().enumerate() {
        //enumerate产生序号i
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) =0xb;
        }
    }

    loop {}
}
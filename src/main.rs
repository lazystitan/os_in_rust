#![no_std] //不使用设计系统调用的标准库
#![no_main]//关闭Rust-Level的入口

use core::panic::PanicInfo;

#[panic_handler]//当Panic时调用，因为原本的panic是rust标准库的一部分
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]//不允许更改函数名
pub extern "C" fn _start() -> ! {
    //此函数是入口，linker会默认寻找一个叫做 "_start" 的函数作为入口
    loop {}
}
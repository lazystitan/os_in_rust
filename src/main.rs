/*
    bios: basic input/output system
    bios boot: bios -> bootable disk -> bootloader (512kb executable) -> kernel image
    vga buffer:25rows, 80columns
*/
#![no_std] //不使用设计系统调用的标准库
#![no_main] //关闭Rust-Level的入口
#![feature(asm)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;
mod serial;

use core::panic::PanicInfo;

#[test_case]
fn trivial_assertion() {
    serial_println!("trivail assertion");
    assert_eq!(0,1);
    serial_println!("[OK]");
}

fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests",tests.len());
    for test in tests {
        test();
    }

    exit_qemu(QemuExitCode::Success);
}

#[cfg(not(test))]//当不是测试时使用
#[panic_handler] //当Panic时调用，因为原本的panic是rust标准库的一部分
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]//是测试时使用
#[panic_handler] //当Panic时调用，因为原本的panic是rust标准库的一部分
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n",info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}



#[no_mangle] //不允许更改函数名
pub extern "C" fn _start() -> ! {
    //此函数是入口，linker会默认寻找一个叫做 "_start" 的函数作为入口
    //    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();//不是错误，见#![reexport_test_harness_main = "test_main"]

    loop {}
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

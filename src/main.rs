#![no_std] //We need a freestanding binary and by default Rust crates link the std libarary thus we use no_std
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(bareMetal_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use bareMetal_os::println;


// mod vga_buffer;
// mod serial;


pub trait Testable {
    fn run (&self) -> ();
}

// impl<T> Testable for T
// where 
//     T: Fn(),

// {
//     fn run(&self) {
//         serial_print!("{}...\t", core::any::type_name::<T>());
//         self();
//         serial_println!("[ok]");
//     }
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// #[repr(u32)]
// pub enum QemuExitCode {
//     Success = 0x10,
//     Failed = 0x11,
// }

// pub fn exit_qemu(exit_code: QemuExitCode) {
//     use x86_64::instructions::port::Port;

//     unsafe {
//         let mut port = Port::new(0xf4);
//         port.write(exit_code as u32);
//     }
// }

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    // loop {}
    bareMetal_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    bareMetal_os::test_panic_handler(_info);
    // serial_println!("[failed]\n");
    // serial_println!("Error: {}\n", _info);
    // exit_qemu(QemuExitCode::Failed);
    // loop {}
}

// static HEY: &[u8] = b"Hello Everyone I am BareMetal-OS Good to see Everyone";

#[no_mangle]

// pub extern "C" fn _start() -> ! {
//     let vga_buffer = 0xb8000 as *mut u8;
//     for(buff, &byte) in HEY.iter().enumerate() {
//         unsafe {
//             *vga_buffer.offset(buff as isize * 2) = byte;
//             *vga_buffer.offset(buff as isize * 2 + 1) = 0xb;
//         }
//     }
//     loop {}
// }

pub extern "C" fn _start() -> ! {
    // vga_buffer::print_on_Display();
    // use core::fmt::Write;
    // vga_buffer::SCREENWRITER.lock().write_str("Hello again").unwrap();
    // write!(vga_buffer::SCREENWRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();
    println!("Hello ,Guys{}", "!");
    //panic!("Some garbage we don't understand :)");
    
    bareMetal_os::init();
    // x86_64::instructions::interrupts::int3();
    
    // unsafe {
    //     *(0xdeadbeef as *mut u8) = 42; //This was to demonstrate double fault
    // }


    // fn stack_overflow() {
    //     stack_overflow();
    // }
    // //Here we trigger the stack overflow
    // stack_overflow();

    #[cfg(test)]
    test_main();
    
    println!("It did not crash!");
    loop {
        bareMetal_os::hlt_loop();
    }
}

// #[cfg(test)]
// pub fn test_runner(tests: &[&dyn Testable]) {

//     serial_println!("Running {} tests", tests.len());
//     for test in tests {
//         test.run();
//     }

//     exit_qemu(QemuExitCode::Success);
// }

//  

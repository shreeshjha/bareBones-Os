use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use lazy_static::lazy_static;
use crate::{print, println};
use crate::gdt;

use pic8259::ChainedPics;
use spin;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]

pub enum InterruptIndex {
	Timer = PIC_1_OFFSET,
	Keyboard,
}

impl InterruptIndex {
	fn as_u8(self) -> u8 {
		self as u8
	}

	fn as_usize(self) -> usize {
		usize::from(self.as_u8())
	}
}
//static IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

// IDT //
lazy_static! {
	static ref IDT: InterruptDescriptorTable = {
		let mut idt = InterruptDescriptorTable::new();
		idt.breakpoint.set_handler_fn(breakpoint_handler);
		unsafe {
			idt.double_fault.set_handler_fn(double_fault_handler)
			.set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
		}
		idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_interrupt_handler);
		idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt_handler);
		idt
	};
}

pub fn init_idt() {
	// let mut idt = InterruptDescriptorTable::new();
	// unsafe {
	// 	IDT.breakpoint.set_handler_fn(breakpoint_handler);
	// 	IDT.load();
	// }
	IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
	println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
	panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

#[test_case]
fn test_breakpoint_exception() {
	x86_64::instructions::interrupts::int3();
}

// PIC //

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> = spin::Mutex::new(unsafe { ChainedPics::
new(PIC_1_OFFSET, PIC_2_OFFSET )});

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
	print!(".");

	unsafe {
		PICS.lock().notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
	}
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
	// print!("k"); works for just getting 1k on the screen we have to use scan for continuos recoginition.

	use x86_64::instructions::port::Port;
	use pc_keyboard::{DecodedKey, HandleControl, Keyboard, layouts, ScancodeSet1};
	use spin::Mutex;

	lazy_static! {
		static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = 
			Mutex::new(Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore));
	}

	let mut keyboard = KEYBOARD.lock();
	let mut port = Port::new(0x60);
	let scancode: u8 = unsafe {
		port.read()
	};
	if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
		if let Some(key) = keyboard.process_keyevent(key_event) {
			match key {
				DecodedKey::Unicode(character) =>  print!{"{}", character},
				DecodedKey::RawKey(key) => print!("{:?}", key),
			}
		}
	}
	// print!("{}", scancode); it lets us print numbers but it won't be the same you press on keyboard random gibberish

	// let numkey = match scancode {
	// 	0x02 => Some('1'),
	// 	 0x03 => Some('2'),
    //     0x04 => Some('3'),
    //     0x05 => Some('4'),
    //     0x06 => Some('5'),
    //     0x07 => Some('6'),
    //     0x08 => Some('7'),
    //     0x09 => Some('8'),
    //     0x0a => Some('9'),
    //     0x0b => Some('0'),
    //     _ => None,
	// };

	// if let Some(numkey) = numkey {
	// 	print!("{}", numkey);
	// } it lets us print only num keys we can do similar with other keys but it will be time consuming and not clean
	//  we use predefined crate called pc-keyboard.
	unsafe {
		PICS.lock().notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
	}
}
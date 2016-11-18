#![feature(lang_items)]
#![feature(const_fn)]
#![feature(unique)]
#![no_std]

extern crate rlibc;

#[macro_use]
mod vga_buffer;

#[no_mangle]
pub extern fn rust_main() {
	let hello = b"Happy 17 Bday!";
	let color_byte = 0x1f; // white foreground, blue background

	let mut hello_colored = [color_byte; 28];
	for (i, char_byte) in hello.into_iter().enumerate() {
		hello_colored[i*2] = *char_byte;
	}

	// write `Hello World!` to the center of the VGA text buffer
	let buffer_ptr = (0xb8000 + 1988) as *mut _;
	unsafe { *buffer_ptr = hello_colored };
	vga_buffer::print_something();
	loop {};

}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
	loop {}
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! { loop{}}


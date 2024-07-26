#![no_std]
#![no_main]
use core::arch::asm;
use core::panic;

// Modules
pub mod memlayout;
pub mod assembly;
pub mod uart;

// Entry Point
#[no_mangle]
extern "C"
fn kmain() {
	let mut my_uart = uart::UartDriver::new(memlayout::UART0);
	my_uart.init();

	println!("Testing IO");
	println!("There is nothing quite like compiler hell!");
}

// Macros 
#[macro_export]
macro_rules! print
{
	($($args:tt)+) => ({
        use core::fmt::Write;

        let _ = write!(crate::uart::UartDriver::new(memlayout::UART0), $($args)+);
	});
}

#[macro_export]
macro_rules! println
{
	() => ({
		print!("\r\n")
	});
	($fmt:expr) => ({
		print!(concat!($fmt, "\r\n"))
	});
	($fmt:expr, $($args:tt)+) => ({
		print!(concat!($fmt, "\r\n"), $($args)+)
	});
}


// Functions 
// #[no_mangle]
// extern "C" fn eh_personality() {}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
	print!("Aborting: ");
	if let Some(p) = info.location() {
		println!(
					"line {}, file {}: {}",
					p.line(),
					p.file(),
					info.message()
		);
	}
	else {
		println!("no information available.");
	}
	abort();
}

#[no_mangle]
extern "C"
fn abort() -> ! {
	loop {
		unsafe {
            // The asm! syntax has changed in Rust.
            // new syntax kicks ass--when we actually get to use it.
			asm!("wfi");
		}
	}
}		




#![no_std]
#![no_main]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_macros)]
#![allow(dead_code)]
use core::arch::asm;

// Modules
pub mod memlayout;
pub mod assembly;
pub mod uart;
pub mod riscv;

// boot.S jumps here after initializing the stack
#[no_mangle]
extern "C"
fn start() {
  // set M Previous Privilege mode to Supervisor, for mret.
  let mut x: u64 = riscv::r_mstatus();
  x = x & !riscv::MSTATUS_MPP_MASK;
  x = x | riscv::MSTATUS_MPP_S;
  riscv::w_mstatus(x);

  // set M Exception Program Counter to main, for mret.
  riscv::w_mepc((kinit as *const ()) as u64);

  // disable paging for now.
  riscv::w_satp(0);

  // delegate all interrupts and exceptions to supervisor mode.
  // riscv::w_medeleg(0xffff);
  // riscv::w_mideleg(0xffff);
  // riscv::w_sie(riscv::r_sie() | riscv::SIE_SEIE | riscv::SIE_STIE | riscv::SIE_SSIE);

  // configure Physical Memory Protection to give supervisor mode
  // access to all of physical memory.
  riscv::w_pmpaddr0(0x3fffffffffffff);
  riscv::w_pmpcfg0(0xf);

  // ask for clock interrupts; TODO: set up later
  timerinit();

  // keep each CPU's hartid in its tp register, for cpuid().
  let id: u64 = riscv::r_mhartid();
  riscv::w_tp(id);

  // switch to supervisor mode and jump to main().
  unsafe {
      asm!("mret");
  }
}


#[no_mangle]
extern "C"
fn kinit() {
	let mut my_uart = uart::UartDriver::new(memlayout::UART0);
	my_uart.init();

	println!("Testing IO");
	println!("There is nothing quite like compiler hell!");
	println!("mhartid: {}", riscv::r_mhartid());

	println!("sp: {}", riscv::r_sp());
    // Test: transmission
    print!("~ ");
    loop {
        if let Some(c) = my_uart.uart_getc() {
            match c {
                0x3 => {
                    println!("");
                    println!("^C: minux exiting");
                    break
                },
                0x0D => { // ANSI for Enter
                    println!("");
                    print!("~ ");
                },
                0x1b => { // Arrow key control
                    if let Some(next_byte) = my_uart.uart_getc() {
                        if next_byte == 91 {
                            // This is a right bracket! We're on our way!
                            if let Some(b) = my_uart.uart_getc() {
                                match b as char {
                                    // eventually make up and down access the buffered history of
                                    // commands
                                    'A' => {
                                        println!("up");
                                    },
                                    'B' => {
                                        println!("down");
                                    },
                                    // likewise for each command, we need to keep track of the
                                    // length so we do not go outside of bounds 
                                    'C' => {
                                        println!("right");
                                    },
                                    'D' => {
                                        println!("left");
                                    },
                                    _ => {
                                        print!("");
                                    }
                                }
                            }
                        }
                    }
                },
                0x08 | 0x7F => { // ANSI for Delete
                    print!("{}{}{}", '\u{0008}', ' ', '\u{0008}');
                },
                _ => {
                    print!("{}", c as char)
                },
            }
        }

    }
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

#[no_mangle]
fn timerinit() {
//   // each CPU has a separate source of timer interrupts.
//   let id: u64 = riscv::r_mhartid();
    // ask the CLINT for a timer interrupt.
//   let interval: u64 = 1000000; // cycles; about 1/10th second in qemu.
//   *(uint64*)CLINT_MTIMECMP(id) = *(uint64*)CLINT_MTIME + interval;
 
    // prepare information in scratch[] for timervec.
    // scratch[0..2] : space for timervec to save registers.
    // scratch[3] : address of CLINT MTIMECMP register.
    // scratch[4] : desired interval (in cycles) between timer interrupts.
//   uint64 *scratch = &timer_scratch[id][0];
//   scratch[3] = CLINT_MTIMECMP(id);
//   scratch[4] = interval;
//   riscv::w_mscratch(scratch as u64);
// 
//   // set the machine-mode trap handler.
    let x: u64;
    unsafe {
        asm!("la {0}, asm_trap_vector ", out(reg) x);
    }
    riscv::w_mtvec(x);

    // enable machine-mode interrupts.
    // riscv::w_mstatus(riscv::r_mstatus() | riscv::MSTATUS_MIE);
 
    // enable machine-mode timer interrupts.
    // riscv::w_mie(riscv::r_mie() | riscv::MIE_MTIE);
}

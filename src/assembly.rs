// assembly.rs
// Assembly imports module
// Source: Stephen Marz
// 20 April 2020

// This came from the Rust book documenting global_asm!. 
// They show using include_str! with it to
// import a full assembly file, which is what I want here.

use core::arch::global_asm;

global_asm!(include_str!("boot.S"));
global_asm!(include_str!("trap.S"));

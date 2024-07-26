use core::arch::asm;

// which hart (core) is this?
pub fn r_mhartid() -> u64 {
    let x: u64;
    unsafe {
        asm!("csrr {0}, mhartid", out(reg) x);
    }
    x
}
// Machine Status Register, mstatus
pub const MSTATUS_MPP_MASK: u64 = 3 << 11; // previous mode.
pub const MSTATUS_MPP_M: u64 = 3 << 11;
pub const MSTATUS_MPP_S: u64 = 1 << 11;
pub const MSTATUS_MPP_U: u64 = 0 << 11;
pub const MSTATUS_MIE: u64 = 1 << 3;    // machine-mode interrupt enable.
                                        
pub fn r_mstatus() -> u64 {
    let x: u64;
    unsafe {
        asm!("csrr {0}, mstatus", out(reg) x);
    }
    x
}

pub fn w_mstatus(x: u64) {
    unsafe {
        asm!("csrw mstatus, {0}", in(reg) x);
    }
}

// machine exception program counter, holds the
// instruction address to which a return from
// exception will go.
pub fn w_mepc(x: u64) {
    unsafe {
        asm!("csrw mepc, {0}", in(reg) x);
    }
}

// Supervisor Status Register, sstatus
pub const SSTATUS_SPP: u64 = 1 << 8;  // Previous mode, 1=Supervisor, 0=User
pub const SSTATUS_SPIE: u64 = 1 << 5; // Supervisor Previous Interrupt Enable
pub const SSTATUS_UPIE: u64 = 1 << 4; // User Previous Interrupt Enable
pub const SSTATUS_SIE: u64 = 1 << 1;  // Supervisor Interrupt Enable
pub const SSTATUS_UIE: u64 = 1 << 0;  // User Interrupt Enable

pub fn r_sstatus() -> u64 {
    let x: u64;
    unsafe {
        asm!("csrr {0}, sstatus", out(reg) x);
    }
    x
}

pub fn w_sstatus(x: u64) {
    unsafe {
        asm!("csrw sstatus, {0}", in(reg) x);
    }
}

// Supervisor Interrupt Pending
pub fn r_sip() -> u64 {
    let x: u64;
    unsafe {
        asm!("csrr {0}, sip", out(reg) x);
    }
    x
}

pub fn w_sip(x: u64) {
    unsafe {
        asm!("csrw sip, {0}", in(reg) x);
    }
}

// Supervisor Interrupt Enable
pub const SIE_SEIE: u64 = 1 << 9; // external
pub const SIE_STIE: u64 = 1 << 5; // timer
pub const SIE_SSIE: u64 = 1 << 1; // software

pub fn r_sie() -> u64 {
    let x: u64;
    unsafe {
        asm!("csrr {0}, sie", out(reg) x);
    }
    x
}

pub fn w_sie(x: u64) {
    unsafe {
        asm!("csrw sie, {0}", in(reg) x);
    }
}

// Machine-mode Interrupt Enable
pub const MIE_MEIE: u64 = 1 << 11; // external
pub const MIE_MTIE: u64 = 1 << 7; // timer
pub const MIE_MSIE: u64 = 1 << 3; // software
                                 
pub fn r_mie() -> u64 {
    let x: u64;
    unsafe {
        asm!("csrr {0}, mie", out(reg) x);
    }
    x
}

pub fn w_mie(x: u64) {
    unsafe {
        asm!("csrw mie, {0}", in(reg) x);
    }
}

// supervisor exception program counter, holds the
// instruction address to which a return from
// exception will go.
pub fn r_sepc() -> u64 {
    let x: u64;
    unsafe {
        asm!("csrr {0}, sepc", out(reg) x);
    }
    x
}

pub fn w_sepc(x: u64) {
    unsafe {
        asm!("csrw sepc, {0}", in(reg) x);
    }
}

// Machine Exception Delegation
pub fn r_medeleg() -> u64 {
    let x: u64;
    unsafe {
        asm!("csrr {0}, medeleg", out(reg) x);
    }
    x
}

pub fn w_medeleg(x: u64) {
    unsafe {
        asm!("csrw medeleg, {0}", in(reg) x);
    }
}

// Machine Interrupt Delegation
pub fn r_mideleg() -> u64 {
    let x: u64;
    unsafe {
        asm!("csrr {0}, mideleg", out(reg) x);
    }
    x
}

pub fn w_mideleg(x: u64) {
    unsafe {
        asm!("csrw mideleg, {0}", in(reg) x);
    }
}

// Supervisor Trap-Vector Base Address
// low two bits are mode.
pub fn w_stvec(x: u64) {
    unsafe {
        asm!("csrw stvec, {0}", in(reg) x);
    }
}

pub fn r_stvec() -> u64 {
    let x: u64; 
    unsafe {
        asm!("csrr {0}, stvec", out(reg) x);
    }
    x
}

// Machine-mode interrupt vector
pub fn w_mtvec(x: u64) {
    unsafe {
        asm!("csrw mtvec, {0}", in(reg) x);
    }
}

// Physical Memory Protection
pub fn w_pmpcfg0(x: u64) {
    unsafe {
        asm!("csrw pmpcfg0, {0}", in(reg) x);
    }
}

pub fn w_pmpaddr0(x: u64) {
    unsafe {
        asm!("csrw pmpaddr0, {0}", in(reg) x);
    }
}

// use riscv's sv39 page table scheme.
pub const SATP_SV39: u64 = 8 << 60;

#[macro_export]
macro_rules! MAKE_SATP{
    ($pagetable:expr) => {
        SATP_SV39 | (($pagetable as u64) >> 12)
    }
}

// supervisor address translation and protection;
// holds the address of the page table.
pub fn w_satp(x: u64) {
    unsafe {
        asm!("csrw satp, {0}", in(reg) x);
    }
}

pub fn r_satp() -> u64 {
    let x: u64;
    unsafe {
        asm!("csrr {0}, satp", out(reg) x);
    }
    x
}

pub fn w_mscratch(x: u64) {
    unsafe {
        asm!("csrw mstratch, {0}", in(reg) x);
    }
}

// Supervisor Trap Cause
pub fn r_scause() {
    let x: u64;
    unsafe {
        asm!("csrr {0}, scause", out(reg) x);
    }
}

// Supervisor Trap Value
pub fn r_stval() {
    let x: u64;
    unsafe {
        asm!("csrr {0}, stval", out(reg) x);
    }
}

// Machine-mode Counter-Enable
pub fn w_mcounteren(x: u64) {
    unsafe {
        asm!("csrw mcounteren, {0}", in(reg) x);
    }
}

pub fn r_mcounteren() {
    let x: u64;
    unsafe {
        asm!("csrr {0}, mcounteren", out(reg) x);
    }
}

// machine-mode cycle counter
pub fn r_time() {
    let x: u64;
    unsafe {
        asm!("csrr {0}, time", out(reg) x);
    }
}

// enable device interrupts
pub fn intr_on() {
    w_sstatus(r_sstatus() | SSTATUS_SIE);
}

pub fn intr_off() {
    w_sstatus(r_sstatus() & !SSTATUS_SIE);
}

// are device interrupts enabled?
pub fn intr_get() -> bool {
    let x: u64 = r_sstatus();
    (x & SSTATUS_SIE) != 0
}

pub fn r_sp() -> u64 {
    let x: u64;
    unsafe {
        asm!("mv {0}, sp", out(reg) x);
    }
    x
}

// read and write tp, the thread pointer, which xv6 uses to hold
// this core's hartid (core number), the index into cpus[].
// static inline uint64
pub fn r_tp() {
    let x: usize;
    unsafe {
        asm!("mv {0}, tp", out(reg) x);
    }
}

pub fn w_tp(x: u64) {
    unsafe {
        asm!("mv tp, {0}", in(reg) x);
    }
}

pub fn r_ra() {
    let x: u64;
    unsafe {
        asm!("mv {0}, ra", out(reg) x);
    }
}

// flush the TLB.
pub fn sfence_vma() {
    // the zero, zero means flush all TLB entries.
    unsafe {
        asm!("sfence.vma zero, zero");
    }
}

/*
 * RISCV-64 PAGE TABLE DEFINITIONS
 */

// typedef uint64 pte_t;
// typedef uint64 *pagetable_t; // 512 PTEs
 
pub const PGSIZE: u64 = 4096; // bytes per page
pub const PGSHIFT: u64 = 12;  // bits of offset within a page
                              
// #define PGROUNDUP(sz)  (((sz)+PGSIZE-1) & ~(PGSIZE-1))
#[macro_export]
macro_rules! PGROUNDUP{
    ($pagetable:expr) => {
        (($pagetable as u64) + PGSIZE - 1) & ~(PGSIZE-1)
    }
}

macro_rules! PGROUNDUP{
    ($a:expr) => {
        (($a as u64) + PGSIZE - 1) & ~(PGSIZE-1)
    }
}

macro_rules! PGROUNDDOWN{
    ($a:expr) => {
        ($a as u64) & ~(PGSIZE-1)
    }
}
 
pub const PTE_V: u64 = 1 << 0; // valid
pub const PTE_R: u64 = 1 << 1;
pub const PTE_W: u64 = 1 << 2;
pub const PTE_X: u64 = 1 << 3;
pub const PTE_U: u64 = 1 << 4; // user can access
 
// shift a physical address to the right place for a PTE.
macro_rules! PA2PTE{
    ($pa:expr) => {
        (($pa as u64) >> 12) << 10
    }
}
 
macro_rules! PTE2PA{
    ($pte:expr) => {
        (($pte as u64) >> 10) << 12
    }
}

macro_rules! PTE_FLAGS{
    ($pte:expr) => {
        ($pte as u64) & 0x3FF
    }
}
 
// extract the three 9-bit page table indices from a virtual address.
pub const PXMASK: u64 = 0x1FF; // 9 bits
macro_rules! PXSHIFT{
    ($level:expr) => {
        PGSHIFT+9*($level as u64) 
    }
}

macro_rules! PX{
    ($level:expr, $va:expr) => {
        (($va as u64) >> PXSHIFT!($level)) & PXMASK
    }
}

// one beyond the highest possible virtual address.
// MAXVA is actually one bit less than the max allowed by
// Sv39, to avoid having to sign-extend virtual addresses
// that have the high bit set.
pub const MAXVA: u64 = 1 << (9 + 9 + 9 + 12 - 1);

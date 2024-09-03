/*
 virt.lds
 Largely Borrowed from Stephen Marz
*/

/*
  riscv is the name of the architecture that the linker understands
  for any RISC-V target (64-bit or 32-bit).

  We will further refine this by using -mabi=lp64 and -march=rv64gc
*/
OUTPUT_ARCH( "riscv" )

ENTRY( _entry )

MEMORY
{
  ram   (wxa) : ORIGIN = 0x80000000, LENGTH = 128M
}

PHDRS
{
  text PT_LOAD;
  data PT_LOAD;
  bss PT_LOAD;
}

SECTIONS
{
  .text : {

    PROVIDE(_text_start = .);
    *(.text.init) *(.text .text.*)

    PROVIDE(_text_end = .);
  } >ram AT>ram :text
   PROVIDE(_global_pointer = .);
  .rodata : {
    PROVIDE(_rodata_start = .);
    *(.rodata .rodata.*)
    PROVIDE(_rodata_end = .);
  } >ram AT>ram :text

  .data : {
	/*
	   . = ALIGN(4096) tells the linker to align the current memory location (which is
	   0x8000_0000 + text section + rodata section) to 4096 bytes. This is because our paging
	   system's resolution is 4,096 bytes or 4 KiB.
	*/
    . = ALIGN(4096);
    PROVIDE(_data_start = .);
	/*
	   sdata and data are essentially the same thing. However, compilers usually use the
	   sdata sections for shorter, quicker loading sections. So, usually critical data
	   is loaded there. However, we're loading all of this in one fell swoop.
	   So, we're looking to put all of the following sections under the umbrella .data:
	   .sdata
	   .sdata.[anything]
	   .data
	   .data.[anything]

	   ...in that order.
	*/
    *(.sdata .sdata.*) *(.data .data.*)
    PROVIDE(_data_end = .);
  } >ram AT>ram :data

  .bss : {
    PROVIDE(_bss_start = .);
    *(.sbss .sbss.*) *(.bss .bss.*)
    PROVIDE(_bss_end = .);
  } >ram AT>ram :bss

  /*
     The following will be helpful when we allocate the kernel stack (_stack) and
	 determine where the heap begnis and ends (_heap_start and _heap_start + _heap_size)/
	 When we do memory allocation, we can use these symbols.

	 We use the symbols instead of hard-coding an address because this is a floating target.
	 As we add code, the heap moves farther down the memory and gets shorter.

	 _memory_start will be set to 0x8000_0000 here. We use ORIGIN(ram) so that it will take
	 whatever we set the origin of ram to. Otherwise, we'd have to change it more than once
	 if we ever stray away from 0x8000_0000 as our entry point.
  */
  PROVIDE(_memory_start = ORIGIN(ram));
  /*
     Our kernel stack starts at the end of the bss segment (_bss_end). However, we're allocating
	 0x80000 bytes (524 KiB) to our kernel stack. This should be PLENTY of space. The reason
	 we add the memory is because the stack grows from higher memory to lower memory (bottom to top).
	 Therefore we set the stack at the very bottom of its allocated slot.
	 When we go to allocate from the stack, we'll subtract the number of bytes we need.
  */
  PROVIDE(_stack = _bss_end + 0x80000);
  PROVIDE(_memory_end = ORIGIN(ram) + LENGTH(ram));

  /* 
     Finally, our heap starts right after the kernel stack. This heap will be used mainly
	 to dole out memory for user-space applications. However, in some circumstances, it will
	 be used for kernel memory as well.

	 We don't align here because we let the kernel determine how it wants to do this.
  */
  PROVIDE(_heap_start = _stack);
  PROVIDE(_heap_size = _memory_end - _stack);
}



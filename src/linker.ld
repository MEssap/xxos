OUTPUT_ARCH(riscv)
ENTRY(_start)
BASE_ADDRESS = 0x80200000;

SECTIONS
{
  . = BASE_ADDRESS;
  skernel = .; /* start of kernel */

  stext = .;   /* start of text section */
  .text : {
    *(.text.entry)
    *(.text .text.*)
    . = ALIGN(0x1000);
    _trampoline = .;
    *(trampsec)
    . = ALIGN(0x1000);
    *(.text .text.*)
  }
  . = ALIGN(0x1000);
  etext = .;   /* end of text section */

  srodata = .; /* start of read-only-data section */
  .rodata : {
    *(.rodata .rodata.*)
    *(.srodata .srodata.*)
  }
  . = ALIGN(0x1000);
  erodata = .; /* end of read-only-data section */

  sdata = .;   /* start of data section */
  .data : {
    *(.data .data.*)
    *(.sdata .sdata.*)
  }
  . = ALIGN(0x1000);
  edata = .;   /* end of data section */

  .bss : {
    *(.bss.stack)
    sbss = .;
    *(.bss .bss.*)
    *(.sbss .sbss.*)
  }
  . = ALIGN(4K);
  ebss = .;    /* end of bss section */

  ekernel = .; /* end of kernel */

  /DISCARD/ : {
    *(.eh_frame)
  }
}

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
    . = ALIGN(0x1000);
    *(.text .text.*)
  }
  . = ALIGN(0x1000);
  etext = .;   /* end of text section */
  . = ALIGN(0x1000);
  srodata = .; /* start of read-only-data section */
  .rodata : {
    *(.rodata .rodata.*)
    *(.srodata .srodata.*)
  }

  . = ALIGN(4K);
  erodata = .; /* end of read-only-data section */
  sdata = .;   /* start of data section */
  .data : {
    *(.data .data.*)
    *(.sdata .sdata.*)
  }

  . = ALIGN(4K);
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

ENTRY(reset_handler)

MEMORY
{
  FLASH : ORIGIN = 0x00000000, LENGTH = 256K
  RAM : ORIGIN = 0x20000000, LENGTH = 64K
}

SECTIONS
{
	.text :
	{
		KEEP(*(.isr_vector))
		*(.text)
		*(.text.*)
    KEEP(*(.usertask.*))
		*(.rodata)
    *(.rodata.*)
		_sidata = .;
	} >FLASH

	.data : AT(_sidata)
	{
		_sdata = .;
		*(.data)
		*(.data*)
		_edata = .;
	} >RAM

	.bss :
	{
		_sbss = .;
		*(.bss)
    *(.bss.*)
		_ebss = .;
	} >RAM

  /DISCARD/ :
  {
    *(.ARM.exidx.*);
  }

	_estack = ORIGIN(RAM) + LENGTH(RAM);
}

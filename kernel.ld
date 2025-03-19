_phys_dram_start = 0x60000000;
_phys_binary_start = 0x80000;

ENTRY(boot)

PHDRS {
    core_management PT_LOAD FLAGS(6);
    boot_stack PT_LOAD FLAGS(6);

    code PT_LOAD FLAGS(5);
    data PT_LOAD FLAGS(6);

    pl011 PT_LOAD FLAGS(6);
    pl111 PT_LOAD FLAGS(6);
}

SECTIONS {
    . = 0x00000000;
    .vector_table : {
        KEEP(*(.text.vector_table))
    }

    . = _phys_dram_start;

    _core_management_start = .;
    .core_management : ALIGN(4) {
        . = ALIGN(1);
        _core_unlock = .;
        . += 1;
        _core_selection = .;
        . += 1;
        _instruction_type = .;
        . += 1;
        _result = .;
        . += 1;
        _core_jump_address = .;
        . += 4;
    } :core_management

    .boot_stack (NOLOAD) : ALIGN(8) {
        _stack_end = .;
        . += 8K;
        _stack_start = .;
    } :boot_stack

    .data : { *(.data*) } :data

    _framebuffer_end = .;
    . += (1200 * 3 * 1200);
    _framebuffer_start = .;

    _global_ram_start = .;
    _global_ram_end = 0x7FFFFFFF;

    . = _phys_binary_start;

    .text : {
        KEEP(*(.text))
        *(.text._start_arguments)
        *(.text._start_rust)
        *(.text*)
        . = ALIGN(8);
    } :code

    .rodata : ALIGN(8) { *(.rodata*) } :code

    .bss (NOLOAD) : ALIGN(16) {
        _bss_start = .;
        *(.bss*);
        . = ALIGN(16);
        _bss_end = .;
    } :data

    .got : { *(.got*) }

    /DISCARD/ : { *(.comment*) }

    . = 0x10004000;
    _pl041_start = .;
    . += 0xfff;
    _pl041_end = .;

    . = 0x10005000;
    _sd_start = .;
    . += 0xfff;
    _sd_end = .;

    . = 0x10006000;
    _ps2_start = .;
    . += 0x1fff;
    _ps2_end = .;

    . = 0x10009000;
    _uart_start = .;

    .pl011 : ALIGN(4) {
        UARTDR = .; . += 0x4;
        UARTRSR = .; . += 0x14;
        UARTFR = .; . += 0x8;
        UARTILPR = .; . += 0x4;
        UARTIBRD = .; . += 0x4;
        UARTFBRD = .; . += 0x4;
        UARTLCR_H = .; . += 0x4;
        UARTCR = .; . += 0x4;
        UARTIFLS = .; . += 0x4;
        UARTIMSC = .; . += 0x4;
        UARTRIS = .; . += 0x4;
        UARTMIS = .; . += 0x4;
        UARTICR = .; . += 0x4;
        UARTDMACR = .; . += 0x4;
    } :pl011

    . += 0xfff;
    _uart_end = .;

    . = 0x10011000;
    _dual_timer_start = .;
    . += 0x1fff;
    _dual_timer_end = .;

    . = 0x10013000;
    _vmmio_start = .;
    . += 0x3fff;
    _vmmio_end = .;

    . = 0x10020000;
    _lcd_start = .;

    .pl111 : ALIGN(4) {
        LCDTiming0 = .; . += 0x4;
        LCDTiming1 = .; . += 0x4;
        LCDTiming2 = .; . += 0x4;
        LCDTiming3 = .; . += 0x4;
        LCDUPBASE = .; . += 0x4;
        LCDLPBASE = .; . += 0x4;
        LCDControl = .; . += 0x4;
        LCDIMSC = .; . += 0x4;
        LCDRIS = .; . += 0x4;
        LCDMIS = .; . += 0x4;
        LCDICR = .; . += 0x4;
    } :pl111

    . += 0xfff;
    _lcd_end = .;
}

_phys_dram_start = 0x60000000;
_phys_binary_start = 0x80000;

ENTRY(boot)

PHDRS {
    vector_table PT_LOAD FLAGS(5);

    core_management PT_LOAD FLAGS(6);
    boot_stack PT_LOAD FLAGS(6);

    code PT_LOAD FLAGS(5);
    data PT_LOAD FLAGS(6);

    pl041 PT_LOAD FLAGS(6);
    pl011 PT_LOAD FLAGS(6);
    pl111 PT_LOAD FLAGS(6);
    pl031 PT_LOAD FLAGS(6);
}

SECTIONS {
    . = 0x00000000;
    .vector_table : {
        KEEP(*(.text.vector_table))
    } :vector_table

    . = _phys_dram_start;

    _core_management_start = .;
    .core_management : ALIGN(4) {
        . = ALIGN(1);
        _core_unlock = .;
        . += 1;
        _core_selection = .;
        . += 1;
        _core_instruction_type = .;
        . += 1;
        _core_result = .;
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
    . += (_framebuffer_width * _framebuffer_height * 4);
    _framebuffer_start = .;

    _framebuffer_width = 1024;
    _framebuffer_height = 768;

    _backbuffer_end = .;
    . += (_framebuffer_width * _framebuffer_height * 4);
    _backbuffer_start = .;

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
        _bss_end = .;
        *(.bss*);
        . = ALIGN(16);
        _bss_start = .;
    } :data

    .got : { *(.got*) }

    /DISCARD/ : { *(.comment*) }

    . = 0x10004000;
    _sound_start = .;
    
    .pl041 : ALIGN(4) {
        AACIRXCR1 = .; . += 4;
        AACITXCR1 = .; . += 4;
        AACISR1 = .; . += 4;
        AACIISR1 = .; . += 4;
        AACIIE1 = .; . += 4;
        AACIRXCR2 = .; . += 4;
        AACITXCR2 = .; . += 4;
        AACISR2 = .; . += 4;
        AACIISR2 = .; . += 4;
        AACIIE2 = .; . += 4;
        AACIRXCR3 = .; . += 4;
        AACITXCR3 = .; . += 4;
        AACISR3 = .; . += 4;
        AACIISR3 = .; . += 4;
        AACIIE3 = .; . += 4;
        AACIRXCR4 = .; . += 4;
        AACITXCR4 = .; . += 4;
        AACISR4 = .; . += 4;
        AACIISR4 = .; . += 4;
        AACIIE4 = .; . += 4;
        AACISL1RX = .; . += 4;
        AACISL1TX = .; . += 4;
        AACISL2RX = .; . += 4;
        AACISL2TX = .; . += 4;
        AACISL12RX = .; . += 4;
        AACISL12TX = .; . += 4;
        AACISLFR = .; . += 4;
        AACISLISTAT = .; . += 4;
        AACISLIEN = .; . += 4;
        AACINTCLR = .; . += 4;
        AACIMAINCR = .; . += 4; 
        AACIRESET = .; . += 4;
        AACISYNC = .; . += 4;
        AACIALLINTS = .; . += 4;
        AACIMAINFR = .; . += 4;
        AACIDR1 = .; . += 4;
        AACIDR2 = .; . += 4;
        AACIDR3 = .; . += 4;
        AACIDR4 = .; . += 4;
    } :pl041

    _sound_end = .;

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

    .pl031 : ALIGN(4) {
        TIMER1Load = .; . += 0x4;
        TIMER1Value = .; . += 0x4;
        TIMER1Control = .; . += 0x4;
        TIMER1IntClr = .; . += 0x4;
        TIMER1RIS = .; . += 0x4;
        TIMER1MIS = .; . += 0x4;
        TIMER1BGLoad = .; . += 0x4;
        TIMER2Load = .; . += 0x4;
        TIMER2Value = .; . += 0x4;
        TIMER2Control = .; . += 0x4;
        TIMER2IntClr = .; . += 0x4;
        TIMER2RIS = .; . += 0x4;
        TIMER2MIS = .; . += 0x4;
        TIMER2BGLoad = .; . += 0x4;
    } :pl031

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

_phys_dram_start = 0x60000000;
_phys_binary_load = 0x80000;
uart_base = 0x10009000;

ENTRY(boot)

PHDRS {
    core_management PT_LOAD FLAGS(6);
    boot_stack PT_LOAD FLAGS(6);

    code PT_LOAD FLAGS(5);
    data PT_LOAD FLAGS(6);
}

SECTIONS {
    . = _phys_dram_start;

    _core_management_base = .;
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

    _global_ram_start = .;
    _global_ram_end = 0x7FFFFFFF;

    . = _phys_binary_load;

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
}

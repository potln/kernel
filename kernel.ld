phys_dram_start = 0x60000000;
phys_binary_load = 0x80000;
uart_base = 0x10009000;
core_management_base = 0x20000000;

ENTRY(boot)

PHDRS {
    boot_stack PT_LOAD FLAGS(6);
    code PT_LOAD FLAGS(5);
    data PT_LOAD FLAGS(6);
    core_management PT_LOAD FLAGS(6);
}

SECTIONS {
    . = phys_dram_start;

    .boot_stack (NOLOAD) : ALIGN(8) {
        _stack_end = .;
        . += 0x1000;
        _stack_start = .;
    } :boot_stack

    . = phys_binary_load;

    .text : {
        KEEP(*(.text))
        *(.text._start_arguments)
        *(.text._start_rust)
        *(.text*)
        . = ALIGN(8);
    } :code

    .rodata : ALIGN(8) { *(.rodata*) } :code

    .data : { *(.data*) } :data

    .bss (NOLOAD) : ALIGN(16) {
        __bss_start = .;
        *(.bss*);
        . = ALIGN(16);
        __bss_end = .;
    } :data

    .got : { *(.got*) }

    . = core_management_base;
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

    /DISCARD/ : { *(.comment*) }
}

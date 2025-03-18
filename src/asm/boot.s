.section .text

boot:
    bl stack_init
    bl bss_init
    bl core_init
    b kernel
.global boot
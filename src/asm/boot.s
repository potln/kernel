.section .text

boot:
    bl stack_init
    bl bss_init
    bl core_sort
    b kernel
.global boot
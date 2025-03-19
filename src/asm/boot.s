.section .text

boot:
    bl stack_init
    bl bss_init
    bl core_init
    bl framebuffer_init
    b kernel
.global boot
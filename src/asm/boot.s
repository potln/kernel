.section .text

boot:
    bl stack_init
    bl bss_init
    bl core_init
    bl fill_framebuffer_white
    b kernel
.global boot
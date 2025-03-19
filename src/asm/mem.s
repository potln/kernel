stack_init:
    ldr sp, =_stack_start
    bic sp, sp, #7
    bx lr
.global stack_init

bss_init:
    ldr r0, =_bss_start
    ldr r1, =_bss_end
    mov r2, #0
bss_clear:
    cmp r0, r1
    it lt
    strlt r2, [r0], #4
    blt bss_clear
    bx lr
.global bss_init

fill_framebuffer_white:
    ldr r0, =_framebuffer_end
    ldr r1, =_framebuffer_start
    mov r2, #0xFFFFFFFF  // White color (assuming 32-bit framebuffer)
fill_loop:
    cmp r0, r1
    it lt
    strlt r2, [r0], #4
    blt fill_loop
    bx lr
.global fill_framebuffer_white
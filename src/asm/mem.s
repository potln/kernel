stack_init:
    ldr sp, =_stack_start
    bic sp, sp, #7
    bx lr
.global stack_init

bss_init:
    ldr r0, =_bss_end
    ldr r1, =_bss_start
    mov r2, #0
bss_clear:
    cmp r0, r1
    it lt
    strlt r2, [r0], #4
    blt bss_clear
    bx lr
.global bss_init

framebuffer_init:
    ldr r0, =_framebuffer_end
    ldr r1, =_framebuffer_start
    mov r2, #0x000000
framebuffer_fill:
    cmp r0, r1
    it lt
    strlt r2, [r0], #4 
    blt framebuffer_fill
    
    ldr r0, =_backbuffer_end
    ldr r1, =_backbuffer_start
    mov r2, #0x000000
backbuffer_fill:
    cmp r0, r1
    it lt
    strlt r2, [r0], #4 
    blt backbuffer_fill
    bx lr
.global framebuffer_init
stack_init:
    ldr sp, =_stack_start
    // Ensure stack is 8-byte aligned
    bic sp, sp, #7
    bx lr
.global stack_init

bss_init:
    ldr r0, =__bss_start
    ldr r1, =__bss_end
    mov r2, #0
bss_clear:
    cmp r0, r1
    it lt
    strlt r2, [r0], #4
    blt bss_clear
    bx lr
.global bss_init
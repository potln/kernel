interrupt_init:
    ldr r0, =vector_table
    mcr p15, 0, r0, c12, c0, 0

    cpsie i
    cpsie f
    bx lr
.global interrupt_init

syscall:
    push {{lr}}
    svc #0
    pop {{lr}}
    bx lr
.global syscall

.section .text.vector_table
vector_table:
    b reset_handler
    b undefined_handler
    b svc_handler
    b prefetch_abort_handler
    b data_abort_handler
    b reserved_handler
    b irq_handler
    b fiq_handler
.global vector_table
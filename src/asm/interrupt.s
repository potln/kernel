interrupt_init:
    cpsie i
    cpsie f
.global interrupt_init

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
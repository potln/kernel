syscall:
    push {{lr}}
    mov r7, r0
    mov r0, r1
    mov r1, r2
    mov r2, r3
    svc #0
    pop {{lr}}
    bx lr
.global syscall
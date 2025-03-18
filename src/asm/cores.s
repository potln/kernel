core_read:
    mrc p15, 0, r0, c0, c0, 5
    and r0, r0, #0x3
    bx lr
.global core_read

core_park:
    wfe
    ldr r1, =_core_unlock
    ldr r0, [r1]
    cmp r0, #1
    bne core_park

    ldr r1, =_core_selection
    bl core_read
    cmp r0, r1
    bne core_park
core_jump:
    ldr r1, =_core_jump_address
    ldr r0, [r1]
    bx r0
.global core_park

core_init:
    push {{lr}}
    bl core_read
    cmp r0, #0
    bne core_park
    pop {{lr}}
    bx lr
.global core_init
    addi    t0, zero, 1
    addi    t1, zero, 1
    addi    t2, zero, 1
    beq     t1, t2, to
    addi    t0, t0, 100
    addi    t0, t0, 200
to:
    add     t0, t0, 1

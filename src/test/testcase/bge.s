_start:
    addi    t0, zero, -1
    addi    t1, zero, 0
    addi    t3, zero, 0
    bge     t0, t1, to1
    addi    t2, t2, 1
to1:
    addi    t2, t2, 10
    bge     t1, t0, to2
    addi    t2, t2, 22
    addi    ra, zero, 0x2C  # label to <to3>
    jalr    zero, ra, 0
to2:
    addi    t2, t2, 13
to3:
    nop

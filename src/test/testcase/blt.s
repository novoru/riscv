_start:
    addi    t0, zero, -1
    addi    t1, zero, 0
    blt     t0, t1, to1
    addi    t2, t2, 1
to1:
    addi    t2, t2, 10
    blt     t1, t0, to2
    addi    t2, t2, 22
    addi    ra, zero, 0x28  # label to <to3>
    jalr    zero, ra, 0
to2:
    addi    t2, t2, 13
to3:
    nop

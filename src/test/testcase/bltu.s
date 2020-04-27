_start:
    addi    t0, zero, 0xFF
    addi    t1, zero, 0
    bltu    t0, t1, to1
    addi    t2, t2, 1
to1:
    addi    t2, t2, 10
    bltu    t1, t0, to2
    addi    t2, t2, 22
    addi    ra, zero, 0x28
    jalr    zero, ra, 0
to2:
    addi    t2, t2, 13
to3:
    nop

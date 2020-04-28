addi    t0, zero, -35       # t0 = -35 = 0xDD
addi    a0, zero, 1000
sb      t0, 0(a0)
lb      t1, 0(a0)

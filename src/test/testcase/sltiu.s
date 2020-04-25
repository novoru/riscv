addi    t0, zero, -2048     # t0 = 0xFFF
sltiu   t1, t0, 2047        # if (t0 < 0x7FF) t1 = 1; else t1 = 0; // result => t1 = 0

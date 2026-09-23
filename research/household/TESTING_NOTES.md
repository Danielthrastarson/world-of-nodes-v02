# B1 implementation observations

Before the experiment matrix, the first 14 new tests passed 13/14. The failing death-custody assertion inspected a zero-quantity historical batch after `move_part`, rather than the positive-quantity descendant. The test was corrected to inspect extant material. Kernel death handling was unchanged. This was a test error, not lost property or a reason to alter inheritance.

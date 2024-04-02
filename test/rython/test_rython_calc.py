from rython import rython_calc

def test_not_overflow_int():
    not_overflow = 2**31 - 1
    not_overflow_int = rython_calc.NewInt(not_overflow)
    assert not_overflow == not_overflow_int.get_number()

def test_overflow_int():
    overflow = 2**31
    overflow_int = rython_calc.NewInt(overflow)
    assert -overflow == overflow_int.get_number()
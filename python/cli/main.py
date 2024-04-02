from time import perf_counter
from rython import rython_calc

if __name__ == "__main__":
    NUMBER_OF_LOOPS = 3 * 10**7

    t1_start = perf_counter()
    new_int: rython_calc.NewInt = rython_calc.NewInt(1)
    for i in range(NUMBER_OF_LOOPS):
        new_int.add(0)
        new_int.mul(1)
    t1_stop = perf_counter()
    print(f"Loop time for NewInt basic methods add and mul: {t1_stop - t1_start}")

    t2_start = perf_counter()
    base_int: int = 1
    for i in range(NUMBER_OF_LOOPS):
        base_int += 0
        base_int *= 1
    t2_stop = perf_counter()
    print(f"Loop time for python int basic methods add and mul: {t2_stop - t2_start}")

    t3_start = perf_counter()
    new_int_2: rython_calc.NewInt = rython_calc.NewInt(1)
    new_int_2.loop_add_mul_seq(NUMBER_OF_LOOPS, 0, 1)
    t3_stop = perf_counter()
    print(f"Loop time for NewInt method with loop in rust: {t3_stop - t3_start}")

    t4_start = perf_counter()
    new_float: rython_calc.NewFloat = rython_calc.NewFloat(1.0)
    for i in range(NUMBER_OF_LOOPS):
        new_float.add(0.0)
        new_float.mul(1.0)
    t4_stop = perf_counter()
    print(f"Loop time for NewFloat basic methods add and mul: {t4_stop - t4_start}")

    t5_start = perf_counter()
    base_float: float = 1.0
    for i in range(NUMBER_OF_LOOPS):
        base_float += 0.0
        base_float *= 1.0
    t5_stop = perf_counter()
    print(f"Loop time for python float basic methods add and mul: {t5_stop - t5_start}")

    t6_start = perf_counter()
    new_float_2: rython_calc.NewFloat = rython_calc.NewFloat(1.0)
    new_float_2.loop_add_mul_seq(NUMBER_OF_LOOPS, 0.0, 1.0)
    t6_stop = perf_counter()
    print(f"Loop time for NewFloat method with loop in rust: {t6_stop - t6_start}")

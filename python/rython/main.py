from time import perf_counter
from rython import calc

if __name__ == "__main__":
    NUMBER_OF_LOOPS = 10**7

    t1_start = perf_counter()
    new_int = calc.NewInt(1)
    for i in range(NUMBER_OF_LOOPS):
        new_int.add(0)
        new_int.mul(1)
    t1_stop = perf_counter()
    print(f"Difference: {t1_stop - t1_start}")

    t2_start = perf_counter()
    base_int = 1
    for i in range(NUMBER_OF_LOOPS):
        base_int += 0
        base_int *= 1
    t2_stop = perf_counter()
    print(f"Difference: {t2_stop - t2_start}")

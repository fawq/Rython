from functools import lru_cache


@lru_cache
def fib_mod_py(index: int, modulo: int) -> int:
    if index == 0:
        return 0
    elif index == 1:
        return 1
    else:
        return (fib_mod_py(index - 1, modulo) + fib_mod_py(index - 2, modulo)) % modulo

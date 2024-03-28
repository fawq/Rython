# Rython

## Use text_signature to create python type

``` rust
/// Formats the sum of two numbers as string.
#[pyfunction(text_signature = "(a: int, b: int) -> str")]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}
```

## Add .pyi stubgen using mypy

In bash type:

``` bash
stubgen -p rython.calc -o python
```

It will produce calc.pyi with:

``` python
def sum_as_string(a: int, b: int) -> str: ...
```

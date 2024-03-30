# Rython

## Use text_signature to create python type

``` rust
/// Formats the sum of two numbers as string.
#[pyfunction(text_signature = "(a: int, b: int) -> str")]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}
```

## Add .pyi file using mypy

Keep in mind that is only works for ```#[pyfunction]```. Macros like ```#[pyclass]``` and ```#[pymethods]``` are still unsupported.

For autocompletion .pyi in bash type:

``` bash
stubgen -p rython.calc -o python
```

It will produce calc.pyi with:

``` python
def sum_as_string(a: int, b: int) -> str: ...
```

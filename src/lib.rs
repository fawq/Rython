use pyo3::prelude::*;

#[pyclass(module = "calc")]
struct NewInt {
    number: i32,
}

#[pymethods]
impl NewInt {
    #[new]
    #[pyo3(text_signature = "(number: int) -> None")]
    fn new(number: i32) -> Self {
        Self { number }
    }

    #[pyo3(text_signature = "($self, second_number: int) -> int")]
    fn mul(&mut self, second_number: i32) -> PyResult<i32> {
        self.number = self.number.wrapping_mul(second_number);
        Ok(self.number)
    }

    #[pyo3(text_signature = "($self, second_number: int) -> int")]
    fn add(&mut self, second_number: i32) -> PyResult<i32> {
        self.number = self.number.wrapping_add(second_number);
        Ok(self.number)
    }

    #[pyo3(text_signature = "($self) -> int")]
    fn get_number(&self) -> PyResult<i32> {
        Ok(self.number)
    }
}

#[pyclass(module = "calc")]
struct NewFloat {
    number: f64,
}

#[pymethods]
impl NewFloat {
    #[new]
    #[pyo3(text_signature = "(number: float) -> None")]
    fn new(number: f64) -> Self {
        Self { number }
    }

    #[pyo3(text_signature = "($self, second_number: float) -> float")]
    fn mul(&mut self, second_number: f64) -> PyResult<f64> {
        self.number *= second_number;
        Ok(self.number)
    }

    #[pyo3(text_signature = "($self, second_number: float) -> float")]
    fn add(&mut self, second_number: f64) -> PyResult<f64> {
        self.number += second_number;
        Ok(self.number)
    }

    #[pyo3(text_signature = "($self) -> float")]
    fn get_number(&self) -> PyResult<f64> {
        Ok(self.number)
    }
}

#[pyfunction]
#[pyo3(text_signature = "(a: int, b: int) -> str")]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pymodule]
fn calc(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<NewInt>()?;
    m.add_class::<NewFloat>()?;
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}

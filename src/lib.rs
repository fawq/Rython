use pyo3::prelude::*;

#[pyclass(module = "calc")]
struct NewInt {
    number: i32,
}

fn wrap(obj: &Bound<'_, PyAny>) -> PyResult<i32> {
    let val = obj.call_method1("__and__", (0xFFFFFFFF_u32,))?;
    let val: u32 = val.extract()?;
    Ok(val as i32)
}

#[pymethods]
impl NewInt {
    #[new]
    #[pyo3(text_signature = "(number: int) -> None")]
    fn new(#[pyo3(from_py_with = "wrap")] number: i32) -> Self {
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

    #[pyo3(
        text_signature = "($self, number_of_loops: int, add_number: int, mul_number: int) -> int"
    )]
    fn loop_add_mul_seq(
        &mut self,
        number_of_loops: i32,
        add_number: i32,
        mul_number: i32,
    ) -> PyResult<i32> {
        for _ in 0..number_of_loops {
            self.number = self.number.wrapping_add(add_number);
            self.number = self.number.wrapping_mul(mul_number);
        }
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

    #[pyo3(
        text_signature = "($self, number_of_loops: int, add_number: float, mul_number: float) -> float"
    )]
    fn loop_add_mul_seq(
        &mut self,
        number_of_loops: i32,
        add_number: f64,
        mul_number: f64,
    ) -> PyResult<f64> {
        for _ in 0..number_of_loops {
            self.number += add_number;
            self.number *= mul_number;
        }
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
fn rython_calc(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<NewInt>()?;
    m.add_class::<NewFloat>()?;
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}

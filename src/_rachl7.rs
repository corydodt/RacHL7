use pyo3::prelude::*;
use pyo3::wrap_pyfunction;


#[pyfunction]
/// Formats the sum of two numbers as string.
fn parse(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}


#[pyfunction]
// hello a world
fn hellow() -> PyResult<()> {
    println!("Hello, world!");
    Ok(())
}


#[pymodule]
/// A Python module implemented in Rust.
fn _rachl7(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(parse))?;
    m.add_wrapped(wrap_pyfunction!(hellow))?;

    Ok(())
}
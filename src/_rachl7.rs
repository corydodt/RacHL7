#[macro_use] extern crate lalrpop_util;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;


lalrpop_mod!(pub calculator1); // synthesized by LALRPOP


#[pyfunction]
/// Parse the string expression and return t/f for whether it is parseable
fn parse(data: &str) -> PyResult<bool> {
    Ok(calculator1::ExprParser::new().parse(data).is_ok())
}


#[pyfunction]
/// Say hello to a world.
fn hellow() -> PyResult<()> {
    println!("Hello, world!");
    Ok(())
}


#[pymodule]
/// A Python module implemented in Rust.
fn _rachl7(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(parse))?;
    m.add_wrapped(wrap_pyfunction!(hellow))?;

    Ok(())
}

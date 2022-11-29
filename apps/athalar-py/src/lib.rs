use athalar_core::from_path;
use pyo3::{pyfunction, pymodule, types::PyModule, wrap_pyfunction, PyResult, Python};

#[pyfunction]
pub fn from_path_py(path: String) -> PyResult<()> {
    let s = from_path(path).expect("oh no!");
    dbg!(&s);
    Ok(())
}

#[pymodule]
fn athalar(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(from_path_py, m)?)?;
    Ok(())
}

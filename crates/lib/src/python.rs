use pyo3::{prelude::*, types::PyModule};

use crate::games;

#[pymodule]
fn gamedig(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(games::minecraft::py_query, m)?)?;
    Ok(())
}

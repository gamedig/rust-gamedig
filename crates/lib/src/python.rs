use pyo3::{prelude::*, types::PyModule};

use crate::games;

#[pymodule]
fn gamedig(_py: Python, m: &PyModule) -> PyResult<()> {
    // Add the game modules.
    let module = PyModule::new(_py, "minecraft")?;
    games::minecraft::minecraft(_py, module)?;
    m.add_submodule(module)?;

    Ok(())
}

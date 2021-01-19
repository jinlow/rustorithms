mod pyquicksort;
mod pyhashtable;
use pyo3::wrap_pymodule;
use pyo3::prelude::*;

#[pymodule]
fn quicksort(_py: Python, m: &PyModule) -> PyResult<()>{
    pyquicksort::quicksort_init(m)?;
    Ok(())
}

#[pymodule]
fn pyorithms(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(quicksort))?;
    m.add_class::<pyhashtable::HashTable>()?;
    Ok(())
}

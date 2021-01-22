mod pyquicksort;
mod pyhashtable;
use pyo3::prelude::*;

// #[pymodule]
// fn quicksort(_py: Python, m: &PyModule) -> PyResult<()>{
//     pyquicksort::quicksort_init(m)?;
//     Ok(())
// }

#[pymodule]
fn pyorithmslib(_py: Python, m: &PyModule) -> PyResult<()> {
    pyquicksort::quicksort_init(m)?;
    m.add_class::<pyhashtable::HashTable>()?;
    Ok(())
}

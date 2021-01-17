use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use rustorithms;

macro_rules! init_qsort_for_type {
    ($name:ident, $type:ty) => {
        #[pyfunction]
        pub fn $name(mut x: Vec<$type>) -> PyResult<Vec<$type>> {
            rustorithms::quicksort(&mut x);
            Ok(x)
        }
    };
}

init_qsort_for_type!(qsort_f32, f32);
init_qsort_for_type!(qsort_f64, f64);
init_qsort_for_type!(qsort_i32, i32);
init_qsort_for_type!(qsort_i64, i64);
init_qsort_for_type!(qsort_str, &str);

pub fn quicksort_init(m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(qsort_f32, m)?)?;
    m.add_function(wrap_pyfunction!(qsort_f64, m)?)?;
    m.add_function(wrap_pyfunction!(qsort_i32, m)?)?;
    m.add_function(wrap_pyfunction!(qsort_i64, m)?)?;
    m.add_function(wrap_pyfunction!(qsort_str, m)?)?;
    Ok(())
}

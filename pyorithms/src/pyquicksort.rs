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

#[derive(FromPyObject)]
pub enum QsInputs {
    #[pyo3(transparent, annotation = "List[str]")]
    Str(Vec<String>),
    #[pyo3(transparent, annotation = "List[int]")]
    Int(Vec<i64>),
    #[pyo3(transparent, annotation = "List[float]")]
    Flt(Vec<f64>),
}

/// Implementation of Quicksort in Rust
#[pyfunction]
pub fn qsort(x: QsInputs) -> PyResult<PyObject> {
    let gil = Python::acquire_gil();
    let py = gil.python();
    match x {
        QsInputs::Int(mut i) => {
            rustorithms::quicksort(&mut i);
            Ok(i.into_py(py))
        }
        QsInputs::Str(mut s) => {
            rustorithms::quicksort(&mut s);
            Ok(s.into_py(py))
        }
        QsInputs::Flt(mut f) => {
            rustorithms::quicksort(&mut f);
            Ok(f.into_py(py))
        }
    }
}

pub fn quicksort_init(m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(qsort, m)?)?;
    // m.add_function(wrap_pyfunction!(qsort_f32, m)?)?;
    // m.add_function(wrap_pyfunction!(qsort_f64, m)?)?;
    // m.add_function(wrap_pyfunction!(qsort_i32, m)?)?;
    // m.add_function(wrap_pyfunction!(qsort_i64, m)?)?;
    // m.add_function(wrap_pyfunction!(qsort_str, m)?)?;
    Ok(())
}

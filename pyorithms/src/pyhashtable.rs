use pyo3::prelude::*;
use std::hash::Hash;
use std::fmt;
use pyo3::PyObjectProtocol;

#[derive(FromPyObject, Hash, std::cmp::PartialEq, std::cmp::Eq)]
pub enum HKey {
    #[pyo3(transparent, annotation = "str")]
    Str(String),
    #[pyo3(transparent, annotation = "int")]
    Int(i64),
}

impl fmt::Display for HKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HKey::Str(s) => write!(f, "'{}'", s),
            HKey::Int(i) => write!(f, "{}", i),
        }
    }
}

#[derive(FromPyObject, std::cmp::PartialEq)]
pub enum HVal {
    #[pyo3(transparent, annotation = "str")]
    Str(String),
    #[pyo3(transparent, annotation = "int")]
    Int(i64),
    #[pyo3(transparent, annotation = "int")]
    Flt(f64),
}

impl fmt::Display for HVal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HVal::Str(s) => write!(f, "'{}'", s),
            HVal::Int(i) => write!(f, "{}", i),
            HVal::Flt(fl) => write!(f, "{}", fl),
        }
    }
}

#[pyclass]
pub struct HashTable {
    ht: rustorithms::Hashtable<HKey, HVal>
}


#[pymethods]
impl HashTable {
    #[new]
    fn new() -> Self {
        HashTable { ht: rustorithms::Hashtable::new() }
    }

    /// Add item to the HashTable
    #[text_signature = "(k, v)"]
    pub fn add(&mut self, k: HKey, v: HVal) -> PyResult<()> {
        self.ht.add(k, v);
        Ok(())
    }

    /// Get HashTable item for a given key.
    #[text_signature = "(k)"]
    pub fn get(&self, py: Python, key: HKey) -> PyResult<PyObject> {
        let v = match self.ht.get(key) {
            Some(val) => val,
            // Returns None if the item isn't in the HashTable
            _ => return Ok(().into_py(py))
        };
        match v {
            HVal::Str(s) => Ok(s.into_py(py)),
            HVal::Int(i) => Ok(i.into_py(py)),
            HVal::Flt(f) => Ok(f.into_py(py)),
        }
    }

    /// Delete item given a specific key
    #[text_signature = "(k)"]
    pub fn delete(&mut self, key: HKey) -> PyResult<()> {
        // TODO: Deal with error
        self.ht.delete(key);
        Ok(())
    }

    /// Print the HashTable
    pub fn print_ht(&self) -> PyResult<()>{
        self.ht.print_ht();
        Ok(())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(self.ht.string_repr())
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(self.ht.string_repr())
    }
}

#[pyproto]
impl PyObjectProtocol for HashTable {
    fn __str__(&self) -> PyResult<String>   {
        Ok(self.ht.string_repr())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(self.ht.string_repr())
    }
}

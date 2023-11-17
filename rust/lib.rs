use pyo3::{pyclass, pymodule, types::PyModule, PyResult, Python, pymethods};

#[pymodule]
#[pyo3(name = "adt_stuff")]
fn py_compnet(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<SimpleStruct>()?;
    m.add_class::<DayOfTheWeek>()?;
    m.add_class::<ComplexEnum>()?;
    Ok(())
}

#[pyclass]
pub struct SimpleStruct {
    pub i: i32,
    pub f: f64,
    pub s: String,
}

#[pymethods]
impl SimpleStruct {
    #[new]
    fn new(i: i32, f: f64, s: String) -> Self {
        Self {
            i,
            f,
            s,
        }
    }
}

#[pyclass]
pub enum ComplexEnum {
    Int { i: i32 },
    Float { f: f64 },
    Str { s: String },
}

#[pyclass]
pub enum DayOfTheWeek {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

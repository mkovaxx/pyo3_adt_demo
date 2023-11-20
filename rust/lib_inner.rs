use pyo3::{
    pyclass, pyfunction, pymethods, pymodule, types::PyModule, wrap_pyfunction, PyResult, Python,
};

#[pymodule]
#[pyo3(name = "adt_stuff")]
fn py_compnet(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<SimpleStruct>()?;
    m.add_class::<DayOfTheWeek>()?;
    m.add_class::<ComplexEnum>()?;
    m.add_wrapped(wrap_pyfunction!(do_stuff))?;
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
        Self { i, f, s }
    }
}

#[pyclass]
pub enum ComplexEnum {
    Int { i: i32 },
    Float { f: f64 },
    Str { s: String },
}

#[pyfunction]
pub fn do_stuff(thing: &ComplexEnum) -> ComplexEnum {
    match thing {
        ComplexEnum::Int { i } => ComplexEnum::Str { s: format!("{i}") },
        ComplexEnum::Float { f } => ComplexEnum::Float { f: f * f },
        ComplexEnum::Str { s } => ComplexEnum::Int { i: s.len() as i32 },
    }
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

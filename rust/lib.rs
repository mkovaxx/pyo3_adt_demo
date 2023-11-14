use pyo3::{pyclass, pymodule, types::PyModule, PyResult, Python};

#[pymodule]
#[pyo3(name = "adt_stuff")]
fn py_compnet(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<DayOfTheWeek>()?;
    Ok(())
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

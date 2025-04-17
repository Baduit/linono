use chrono::NaiveDate;
use std::collections::BTreeMap;

use pyo3::prelude::*;
use linono_extractor::{Release, Releases};

#[pyclass]
struct PyRelease(Release);

#[pymethods]
impl PyRelease {
    #[getter]
    fn saga(&self) -> &str {
        &self.0.saga
    }

    #[getter]
    fn title(&self) -> &str {
        &self.0.title
    }

    #[getter]
    fn release_date(&self) -> &Option<NaiveDate> {
        &self.0.release_date
    }
}

#[pyclass]
struct PyReleases(Releases);

#[pymethods]
impl PyReleases {
    #[staticmethod]
    fn load() -> PyResult<Self> {
        match Releases::load() {
            Ok(releases) => Ok(PyReleases(releases)),
            Err(err) => Err(pyo3::exceptions::PyRuntimeError::new_err(err.to_string())),
        }
    }

    fn coming(&self) -> Vec<PyRelease> {
        self.0
            .coming
            .iter()
            .cloned()
            .map(PyRelease)
            .collect()
    }

    fn all(&self) -> BTreeMap<String, Vec<PyRelease>> {
        self.0
            .all
            .iter()
            .map(|(key, releases)| {
                (
                    key.clone(),
                    releases.iter().cloned().map(PyRelease).collect(),
                )
            })
            .collect()
    }
}

#[pymodule]
fn linono_pyextractor(m: &Bound<'_, PyModule>) -> PyResult<()> {
    pyo3_log::init();
    m.add_class::<PyRelease>()?;
    m.add_class::<PyReleases>()?;
    Ok(())
}
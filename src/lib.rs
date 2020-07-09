use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::PyIterProtocol;
use pyo3::PyObjectProtocol;

extern crate las;
use crate::las::Read;

/// A LAS point
#[pyclass(unsendable)]
pub struct LASpoint {
    #[pyo3(get)]
    x: f64,
    #[pyo3(get)]
    y: f64,
    #[pyo3(get)]
    z: f64,
    #[pyo3(get)]
    intensity: u16,
}

#[pyclass(unsendable)]
#[derive(Clone)]
pub struct LASheader {
    #[pyo3(get)]
    number_of_points: u64,
    #[pyo3(get)]
    version: String,
}

#[pyclass(unsendable)]
struct LASdataset {
    r: las::Reader,
    a1: Vec<u32>,
}

#[pymethods]
impl LASdataset {
    fn number_of_points(&self) -> PyResult<u64> {
        Ok(self.r.header().number_of_points())
    }
    fn version(&self) -> PyResult<String> {
        let strv = format!(
            "{}.{}",
            self.r.header().version().major,
            self.r.header().version().minor
        );
        Ok(strv)
    }
    #[getter]
    fn header(&self) -> PyResult<LASheader> {
        let h = LASheader {
            number_of_points: self.r.header().number_of_points(),
            version: "1.4".to_string(),
        };
        Ok(h)
    }
    fn next_point(&mut self) -> PyResult<LASpoint> {
        let re = self.r.read();
        if re.is_none() {
            return Err(PyErr::new::<exceptions::IOError, _>(
                "Invalid path for LAS/LAZ file.",
            ));
        }
        let p = re.unwrap().unwrap();
        let p2 = LASpoint {
            x: p.x,
            y: p.y,
            z: p.z,
            intensity: p.intensity,
        };
        Ok(p2)
    }
}

#[pyproto]
impl PyObjectProtocol for LASdataset {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!("Dataset hugo str"))
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Dataset hugo repr"))
    }
}

#[pyclass]
struct Iter {
    inner: std::vec::IntoIter<u32>,
}

#[pyproto]
impl PyIterProtocol for Iter {
    fn __iter__(slf: PyRefMut<Self>) -> Py<Iter> {
        slf.into()
    }

    fn __next__(mut slf: PyRefMut<Self>) -> Option<u32> {
        slf.inner.next()
    }
}

#[pyproto]
impl PyIterProtocol for LASdataset {
    fn __iter__(slf: PyRef<Self>) -> PyResult<Py<Iter>> {
        let iter = Iter {
            inner: slf.a1.clone().into_iter(),
        };
        Py::new(slf.py(), iter)
    }
}

/// testing
#[pyfunction]
fn read_file(path: String) -> PyResult<LASdataset> {
    let re = las::Reader::from_path(path);
    if re.is_err() {
        return Err(PyErr::new::<exceptions::IOError, _>(
            "Invalid path for LAS/LAZ file.",
        ));
    }
    let ds = re.unwrap();
    Ok(LASdataset {
        r: ds,
        a1: vec![2, 5, 9],
    })
}

#[pymodule]
fn simplaz(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<LASdataset>()?;
    m.add_class::<LASpoint>()?;
    m.add_class::<LASheader>()?;
    m.add_wrapped(wrap_pyfunction!(read_file)).unwrap();
    Ok(())
}

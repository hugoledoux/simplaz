use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

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
    #[pyo3(get)]
    header: LASheader,
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
    fn read(&mut self) -> PyResult<LASpoint> {
        let p = self.r.read().unwrap().unwrap();
        let p2 = LASpoint {
            x: p.x,
            y: p.y,
            z: p.z,
            intensity: p.intensity,
        };
        Ok(p2)
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
    let h = LASheader {
        number_of_points: ds.header().number_of_points(),
        version: "1.4".to_string(),
    };
    let tmp = LASdataset { r: ds, header: h };
    Ok(tmp)
}

#[pymodule]
fn simplaz(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<LASdataset>()?;
    m.add_class::<LASpoint>()?;
    m.add_wrapped(wrap_pyfunction!(read_file)).unwrap();
    Ok(())
}

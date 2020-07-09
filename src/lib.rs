use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

extern crate las;
use crate::las::Read;

#[pyclass(unsendable)]
struct LASdataset {
    r: las::Reader,
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
    fn read(&mut self) -> PyResult<f64> {
        let p = self.r.read().unwrap().unwrap();
        Ok(p.x)
    }
}

// #[pyclass(unsendable)]
// struct LASheader {
//     #[pyo3(get)]
//     bounds: u32,
//     #[pyo3(get)]
//     count: usize,
// }

/// testing
#[pyfunction]
fn read_file(path: String) -> PyResult<LASdataset> {
    let re = las::Reader::from_path(path);
    if re.is_err() {
        return Err(PyErr::new::<exceptions::IOError, _>(
            "Invalid path for LAS/LAZ file.",
        ));
    }
    let tmp = LASdataset { r: re.unwrap() };
    Ok(tmp)
}

#[pymodule]
fn simplaz(_py: Python, m: &PyModule) -> PyResult<()> {
    // m.add_wrapped(wrap_pyfunction!(double)).unwrap();
    // m.add_wrapped(wrap_pyfunction!(read_test)).unwrap();
    // m.add_wrapped(wrap_pyfunction!(read_las1)).unwrap();
    m.add_wrapped(wrap_pyfunction!(read_file)).unwrap();
    Ok(())
}

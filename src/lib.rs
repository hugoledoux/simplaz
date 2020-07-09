use pyo3::class::iter::IterNextOutput;
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
    #[pyo3(get)]
    return_number: u8,
    #[pyo3(get)]
    number_of_returns: u8,
    // #[pyo3(get)]
    // scan_direction: ScanDirection,
    #[pyo3(get)]
    is_edge_of_flight_line: bool,
    #[pyo3(get)]
    classification: u8,
    #[pyo3(get)]
    is_synthetic: bool,
    #[pyo3(get)]
    is_key_point: bool,
    #[pyo3(get)]
    is_withheld: bool,
    #[pyo3(get)]
    is_overlap: bool,
    #[pyo3(get)]
    scanner_channel: u8,
    #[pyo3(get)]
    scan_angle: f32,
    #[pyo3(get)]
    user_data: u8,
    #[pyo3(get)]
    point_source_id: u16,
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
    count: usize,
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
            return_number: p.return_number,
            number_of_returns: p.number_of_returns,
            // scan_direction: p.scan_direction,
            is_edge_of_flight_line: p.is_edge_of_flight_line,
            classification: u8::from(p.classification),
            is_synthetic: p.is_synthetic,
            is_key_point: p.is_key_point,
            is_withheld: p.is_withheld,
            is_overlap: p.is_overlap,
            scanner_channel: p.scanner_channel,
            scan_angle: p.scan_angle,
            user_data: p.user_data,
            point_source_id: p.point_source_id,
        };
        Ok(p2)
    }
}

#[pyproto]
impl PyIterProtocol for LASdataset {
    fn __next__(mut slf: PyRefMut<Self>) -> IterNextOutput<usize, &'static str> {
        if slf.count < 5 {
            slf.count += 1;
            IterNextOutput::Yield(slf.count)
        } else {
            IterNextOutput::Return("Ended")
        }
    }
    // fn __iter__(mut slf: PyRefMut<Self>) -> Self {
    //     self
    // }
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
    Ok(LASdataset { r: ds, count: 0 })
}

#[pymodule]
fn simplaz(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<LASdataset>()?;
    m.add_class::<LASpoint>()?;
    m.add_class::<LASheader>()?;
    m.add_wrapped(wrap_pyfunction!(read_file)).unwrap();
    Ok(())
}

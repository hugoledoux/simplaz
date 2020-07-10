use pyo3::class::iter::IterNextOutput;
use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::PyIterProtocol;
use pyo3::PyObjectProtocol;

extern crate las;
use crate::las::Read;

/// A LAZ/LAS point
#[pyclass(unsendable)]
struct LazPoint {
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
    // scan_direction: ScanDirection, TODO
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

#[pyproto]
impl PyObjectProtocol for LazPoint {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!("({}, {}, {})", self.x, self.y, self.z))
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("({}, {}, {})", self.x, self.y, self.z))
    }
}

#[pyclass(unsendable)]
#[derive(Clone)]
struct LazHeader {
    #[pyo3(get)]
    number_of_points: u64,
    #[pyo3(get)]
    version: String,
    #[pyo3(get)]
    system_identifier: String,
    #[pyo3(get)]
    scale: Vec<f64>,
    #[pyo3(get)]
    offset: Vec<f64>,
    #[pyo3(get)]
    bounds: Vec<f64>,
    #[pyo3(get)]
    point_format: u8,
}

/// a LazDataset is bla bla bla
#[pyclass(unsendable)]
struct LazDataset {
    r: las::Reader,
}

#[pymethods]
impl LazDataset {
    ///  0 => CreatedNeverClassified
    ///  1 => Unclassified
    ///  2 => Ground
    ///  3 => LowVegetation
    ///  4 => MediumVegetation
    ///  5 => HighVegetation
    ///  6 => Building
    ///  7 => LowPoint
    ///  8 => ModelKeyPoint
    ///  9 => Water
    /// 10 => Rail
    /// 11 => RoadSurface
    /// 12 => Error::OverlapClassification
    /// 13 => WireGuard
    /// 14 => WireConductor
    /// 15 => TransmissionTower
    /// 16 => WireStructureConnector
    /// 17 => BridgeDeck
    /// 18 => HighNoise
    #[getter]
    fn header(&self) -> PyResult<LazHeader> {
        let strv = format!(
            "{}.{}",
            self.r.header().version().major,
            self.r.header().version().minor
        );
        let h = LazHeader {
            number_of_points: self.r.header().number_of_points(),
            version: strv,
            system_identifier: self.r.header().system_identifier().to_string(),
            point_format: self.r.header().point_format().to_u8().unwrap(),
            scale: vec![
                self.r.header().transforms().x.scale,
                self.r.header().transforms().y.scale,
                self.r.header().transforms().z.scale,
            ],
            offset: vec![
                self.r.header().transforms().x.offset,
                self.r.header().transforms().y.offset,
                self.r.header().transforms().z.offset,
            ],
            bounds: vec![
                self.r.header().bounds().min.x,
                self.r.header().bounds().min.y,
                self.r.header().bounds().min.z,
                self.r.header().bounds().max.x,
                self.r.header().bounds().max.y,
                self.r.header().bounds().max.z,
            ],
        };
        Ok(h)
    }
    fn all_points(&mut self) -> PyResult<Vec<LazPoint>> {
        let mut ls: Vec<LazPoint> = Vec::new();
        for each in self.r.points() {
            let p = each.unwrap();
            let p2 = LazPoint {
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
            ls.push(p2);
        }
        // let _re = self.r.seek(0); TODO how to reset to start the iterator?
        Ok(ls)
    }
}

#[pyproto]
impl PyIterProtocol for LazDataset {
    fn __next__(mut slf: PyRefMut<Self>) -> IterNextOutput<LazPoint, &'static str> {
        let re = slf.r.read();
        if re.is_none() {
            return IterNextOutput::Return("Ended");
        }
        let p = re.unwrap().unwrap();
        let p2 = LazPoint {
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
        IterNextOutput::Yield(p2)
    }
    fn __iter__(slf: PyRefMut<Self>) -> Py<LazDataset> {
        slf.into()
    }
}

#[pyproto]
impl PyObjectProtocol for LazDataset {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!(
            "v{}.{}; {} points, PointFormat({})",
            self.r.header().version().major,
            self.r.header().version().minor,
            self.r.header().number_of_points(),
            self.r.header().point_format().to_u8().unwrap()
        ))
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "v{}.{}; {} points, PointFormat({})",
            self.r.header().version().major,
            self.r.header().version().minor,
            self.r.header().number_of_points(),
            self.r.header().point_format().to_u8().unwrap()
        ))
    }
}

/// Read a LAZ/LAS file and return a LazDataset object
#[pyfunction]
fn read_file(path: String) -> PyResult<LazDataset> {
    let re = las::Reader::from_path(path);
    if re.is_err() {
        return Err(PyErr::new::<exceptions::IOError, _>(
            "Invalid path for LAS/LAZ file.",
        ));
    }
    let ds = re.unwrap();
    Ok(LazDataset { r: ds })
}

#[pymodule]
fn simplaz(_py: Python, m: &PyModule) -> PyResult<()> {
    // m.add_class::<LazDataset>()?;
    // m.add_class::<LazPoint>()?;
    // m.add_class::<LazHeader>()?;
    m.add_wrapped(wrap_pyfunction!(read_file)).unwrap();
    Ok(())
}

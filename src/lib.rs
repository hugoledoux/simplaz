use crate::chrono::Datelike;
use chrono::offset::Utc;
use chrono::prelude::Date;

use pyo3::class::iter::IterNextOutput;
use pyo3::exceptions;
use pyo3::prelude::*;

use pyo3::wrap_pyfunction;
use pyo3::PyIterProtocol;
use pyo3::PyObjectProtocol;

extern crate chrono;

extern crate las;
use crate::las::Read;

#[pyclass(unsendable)]
/// A LAZ/LAS point with all its attributes
pub struct LazPoint {
    //-- point_format: 0
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
    scan_direction_flag: bool,
    #[pyo3(get)]
    number_of_returns: u8,
    #[pyo3(get)]
    edge_of_flight_line: bool,
    #[pyo3(get)]
    classification: u8,
    #[pyo3(get)]
    scan_angle_rank: f32,
    #[pyo3(get)]
    user_data: u8,
    #[pyo3(get)]
    point_source_id: u16,
    //-- Classification Bit Field Encoding
    #[pyo3(get)]
    is_synthetic: bool,
    #[pyo3(get)]
    is_key_point: bool,
    #[pyo3(get)]
    is_withheld: bool,
    #[pyo3(get)]
    is_overlap: bool,
    //---------- point_format: 1
    #[pyo3(get)]
    gps_time: Option<f64>,
    //---------- point_format: 2
    #[pyo3(get)]
    scanner_channel: Option<u8>,
    #[pyo3(get)]
    color: Option<(u16, u16, u16)>,
    #[pyo3(get)]
    waveform: Option<LazWaveform>,
    #[pyo3(get)]
    nir: Option<u16>,
}

#[pyproto]
impl PyObjectProtocol for LazPoint {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!(
            "({}, {}, {}) | classification: {} | intensity: {}",
            self.x, self.y, self.z, self.classification, self.intensity
        ))
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "({}, {}, {}) | classification: {} | intensity: {}",
            self.x, self.y, self.z, self.classification, self.intensity
        ))
    }
}

#[pyclass(unsendable)]
#[derive(Clone)]
pub struct LazWaveform {
    #[pyo3(get)]
    wave_packet_descriptor_index: u8,
    #[pyo3(get)]
    byte_offset_to_waveform_data: u64,
    #[pyo3(get)]
    waveform_packet_size_in_bytes: u32,
    #[pyo3(get)]
    return_point_waveform_location: f32,
    #[pyo3(get)]
    x_t: f32,
    #[pyo3(get)]
    y_t: f32,
    #[pyo3(get)]
    z_t: f32,
}

#[pyclass(unsendable)]
#[derive(Clone)]
/// the LAS Header object
struct LazHeader {
    #[pyo3(get)]
    number_of_points: u64,
    #[pyo3(get)]
    version: String,
    #[pyo3(get)]
    system_identifier: String,
    #[pyo3(get)]
    scale_factor: Vec<f64>,
    #[pyo3(get)]
    offset: Vec<f64>,
    #[pyo3(get)]
    bounds: Vec<f64>,
    #[pyo3(get)]
    point_format: u8,
    #[pyo3(get)]
    generating_software: String,
    #[pyo3(get)]
    number_of_points_by_return: Vec<u64>,
    //-----
    file_creation_date: Date<Utc>,
}

#[pymethods]
impl LazHeader {
    #[getter]
    /// returns (year, month, day)
    fn file_creation_date(&self) -> PyResult<(i32, u32, u32)> {
        Ok((
            self.file_creation_date.naive_utc().year(),
            self.file_creation_date.naive_utc().month(),
            self.file_creation_date.naive_utc().day(),
        ))
    }
}

#[pyclass(unsendable)]
/// a LazDataset contains a LazHeader and a series of LazPoint
struct LazDataset {
    r: las::Reader,
}

#[pymethods]
impl LazDataset {
    #[getter]
    /// Returns the LazHeader object
    fn header(&self) -> PyResult<LazHeader> {
        let strv = format!(
            "{}.{}",
            self.r.header().version().major,
            self.r.header().version().minor
        );
        let d = if self.r.header().date().is_none() {
            Utc::today()
        } else {
            self.r.header().date().unwrap()
        };
        let mut nppr: Vec<u64> = Vec::new();
        for i in 1..15 {
            let c = self.r.header().number_of_points_by_return(i);
            if c.is_none() {
                break;
            }
            nppr.push(c.unwrap());
        }
        let h = LazHeader {
            number_of_points: self.r.header().number_of_points(),
            version: strv,
            file_creation_date: d,
            generating_software: self.r.header().generating_software().to_string(),
            system_identifier: self.r.header().system_identifier().to_string(),
            point_format: self.r.header().point_format().to_u8().unwrap(),
            number_of_points_by_return: nppr,
            scale_factor: vec![
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
    /// Returns all points (thus could take a lot of memory)
    /// using the iterator for LazDataset (`iter(ds)`) avoids this
    fn all_points(&mut self) -> PyResult<Vec<LazPoint>> {
        let mut ls: Vec<LazPoint> = Vec::new();
        for each in self.r.points() {
            let p = each.unwrap();
            let p2 = make_lazpoint(&p);
            ls.push(p2);
        }
        let _re = self.r.seek(0);
        Ok(ls)
    }
    /// To move the iterator to a specific point (0-indexed)
    /// It should be a value [0, total-1]
    fn seek(&mut self, pos: u64) {
        let _re = self.r.seek(pos);
    }
}

#[pyproto]
impl PyIterProtocol for LazDataset {
    fn __next__(mut slf: PyRefMut<Self>) -> IterNextOutput<LazPoint, &'static str> {
        let re = slf.r.read();
        if re.is_none() {
            let _re = slf.r.seek(0);
            return IterNextOutput::Return("Ended");
        }
        let p = re.unwrap().unwrap();
        let p2 = make_lazpoint(&p);
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
    m.add_class::<LazDataset>()?;
    m.add_class::<LazPoint>()?;
    m.add_class::<LazHeader>()?;
    m.add_wrapped(wrap_pyfunction!(read_file)).unwrap();
    Ok(())
}

fn make_lazpoint(p: &las::point::Point) -> LazPoint {
    LazPoint {
        x: p.x,
        y: p.y,
        z: p.z,
        intensity: p.intensity,
        return_number: p.return_number,
        number_of_returns: p.number_of_returns,
        scan_direction_flag: if p.scan_direction == las::point::ScanDirection::LeftToRight {
            true
        } else {
            false
        },
        edge_of_flight_line: p.is_edge_of_flight_line,
        classification: u8::from(p.classification),
        scan_angle_rank: p.scan_angle,
        user_data: p.user_data,
        point_source_id: p.point_source_id,
        is_synthetic: p.is_synthetic,
        is_key_point: p.is_key_point,
        is_withheld: p.is_withheld,
        is_overlap: p.is_overlap,
        // scanner_channel: p.scanner_channel,
        gps_time: p.gps_time,
        scanner_channel: Some(p.scanner_channel),
        color: if p.color.is_some() {
            Some((
                p.color.unwrap().red,
                p.color.unwrap().green,
                p.color.unwrap().blue,
            ))
        } else {
            None
        },
        waveform: if p.waveform.is_some() {
            Some(LazWaveform {
                wave_packet_descriptor_index: p.waveform.unwrap().wave_packet_descriptor_index,
                byte_offset_to_waveform_data: p.waveform.unwrap().byte_offset_to_waveform_data,
                waveform_packet_size_in_bytes: p.waveform.unwrap().waveform_packet_size_in_bytes,
                return_point_waveform_location: p.waveform.unwrap().return_point_waveform_location,
                x_t: p.waveform.unwrap().x_t,
                y_t: p.waveform.unwrap().y_t,
                z_t: p.waveform.unwrap().z_t,
            })
        } else {
            None
        },
        nir: p.nir,
    }
}

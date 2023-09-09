use std::path::PathBuf;

use arrow::record_batch::RecordBatch;

fn write_arrow_table(table: RecordBatch, to: PathBuf) {
    let writer = std::fs::File::create(to).unwrap();
    let mut arrow_writer = arrow::csv::WriterBuilder::new()
        .has_headers(true)
        .build(writer);
    arrow_writer.write(&table).unwrap();
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.

mod python {
    use super::RecordBatch;
    use pyo3::prelude::*;

    /// defines the python module to import
    #[pymodule]
    fn apache_arrow_py_rust_demo(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
        m.add_function(wrap_pyfunction!(write_arrow_table, m)?)?;
        Ok(())
    }

    /// wrapper around crate::write_arrow_table
    #[pyfunction]
    fn write_arrow_table(
        py_table: arrow::pyarrow::PyArrowType<RecordBatch>,
        to: std::path::PathBuf,
    ) -> PyResult<()> {
        let table = py_table.0;
        super::write_arrow_table(table, to);

        Ok(())
    }
}

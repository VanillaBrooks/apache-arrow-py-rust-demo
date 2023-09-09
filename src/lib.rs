use std::path::PathBuf;

use arrow::record_batch::RecordBatch;

/// pure rust function. Tables an apache arrow RecordBatch and writes it to the specified path
fn write_arrow_table(table: RecordBatch, to: PathBuf) {
    let writer = std::fs::File::create(to).unwrap();

    let mut arrow_writer = arrow::csv::WriterBuilder::new()
        .has_headers(true)
        .build(writer);

    arrow_writer.write(&table).unwrap();
}

/// container for all python code
mod python {
    use super::RecordBatch;
    use pyo3::prelude::*;

    /// defines the python module to import
    #[pymodule]
    fn apache_arrow_py_rust_demo(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
        m.add_function(wrap_pyfunction!(write_arrow_table, m)?)?;
        Ok(())
    }

    /// wrapper around crate::write_arrow_table. This is a function exported to python
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

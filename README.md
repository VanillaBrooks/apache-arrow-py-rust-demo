# apache-arrow-py-rust-demo

illustrate passing python columnar data stores (e.g.: pandas dataframes) to rust
through apache arrow and `pyo3`


## Building

* Install `maturin` 
    * `cargo install maturin`
    * `pip install maturin`
* Install the rust compiler & package manager `cargo`
* Python dependencies for the demo:
    * `pandas` `pyarrow`

Clone the repo

```
git clone https://github.com/VanillaBrooks/apache-arrow-py-rust-demo.git --depth 1
cd apache-arrow-py-rust-demo
```

Build the rust code to consume from python:

```bash
pip install .
```


## Usage

now, from python we can import `apache_arrow_py_rust_demo` as a library. This library contains one 
function: `write_arrow_table` that takes a [`pyarrow.RecordBatch`](https://arrow.apache.org/docs/python/generated/pyarrow.RecordBatch.html)
(trivially easy to construct from a `pandas` dataframe), and the path to write the data to.

## Example

An example script is supplied in `./execute_demo.py`. This script reads the input table `input_table.csv`
to a dataframe, converts the dataframe to a `pyarrow.RecordBatch`, and then passes the data to rust. To
demonstrate the data is identical, the rust code writes the data to the supplied argument (`output_table.csv`).

The script is executed with:


```bash
python3 ./execute_demo.py
```

## Results

```
bat input_table.csv
───────┬────────────────────────────────
       │ File: input_table.csv
───────┼────────────────────────────────
   1   │ int_col,float_col,string_col
   2   │ 1,3.1,"row_1"
   3   │ 2,2.7,"row_2"
   4   │ 3,1.9,"row_3"
   5   │ 9,8.0,"row_4"
───────┴────────────────────────────────
```

```
bat output_table.csv
───────┬────────────────────────────────
       │ File: output_table.csv
───────┼────────────────────────────────
   1   │ int_col,float_col,string_col
   2   │ 1,3.1,row_1
   3   │ 2,2.7,row_2
   4   │ 3,1.9,row_3
   5   │ 9,8.0,row_4
───────┴────────────────────────────────
```

there is no data loss when passing the tables to rust

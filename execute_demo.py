import os

import apache_arrow_py_rust_demo as demo
import pandas as pd
import pyarrow as pa

input_path = "input_table.csv"
output_path = "output_table.csv"

#
# remove the output CSV if it exists
#
if os.path.exists(output_path):
    os.remove(output_path)

#
# load the input CSV with pandas
#
input_df = pd.read_csv(input_path)
print("input dataframe: \n", input_df.head())

# convert the pandas dataframe to apache arrow RecordBatch.
# it seems at this point rust does not have support for arrow tables (?)
arrow_records = pa.RecordBatch.from_pandas(input_df)
print("\narrow record batch:\n", arrow_records)

#
# call our rust function
#
demo.write_arrow_table(arrow_records, output_path)

#
# show that the data was written:
#
assert os.path.exists(output_path) == True
output_df = pd.read_csv(output_path)
print("\noutput dataframe:\n", output_df.head())

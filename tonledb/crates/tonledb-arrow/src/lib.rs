//! Arrow and Parquet support for TonleDB

use arrow::array::{ArrayRef, Int64Array, Float64Array, StringArray, BooleanArray};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use parquet::arrow::ArrowWriter;
use parquet::file::properties::WriterProperties;
use parquet::file::writer::InMemoryWriteableCursor;
use std::sync::Arc;
use tonledb_core::{Db, DbError, Result, Space, Storage, Value};

/// Convert TonleDB values to Arrow arrays
pub fn values_to_arrow_arrays(values: &[Value]) -> Result<Vec<ArrayRef>> {
    if values.is_empty() {
        return Ok(vec![]);
    }
    
    // Determine the data type from the first value
    let data_type = match &values[0] {
        Value::I64(_) => DataType::Int64,
        Value::F64(_) => DataType::Float64,
        Value::Str(_) => DataType::Utf8,
        Value::Bool(_) => DataType::Boolean,
        Value::Null => DataType::Null,
        _ => return Err(DbError::Invalid("Unsupported data type for Arrow conversion".into())),
    };
    
    // Create the appropriate array based on the data type
    let array: ArrayRef = match data_type {
        DataType::Int64 => {
            let vals: Vec<Option<i64>> = values.iter().map(|v| {
                match v {
                    Value::I64(val) => Some(*val),
                    Value::Null => None,
                    _ => None,
                }
            }).collect();
            Arc::new(Int64Array::from(vals))
        }
        DataType::Float64 => {
            let vals: Vec<Option<f64>> = values.iter().map(|v| {
                match v {
                    Value::F64(val) => Some(*val),
                    Value::Null => None,
                    _ => None,
                }
            }).collect();
            Arc::new(Float64Array::from(vals))
        }
        DataType::Utf8 => {
            let vals: Vec<Option<String>> = values.iter().map(|v| {
                match v {
                    Value::Str(val) => Some(val.clone()),
                    Value::Null => None,
                    _ => None,
                }
            }).collect();
            Arc::new(StringArray::from(vals))
        }
        DataType::Boolean => {
            let vals: Vec<Option<bool>> = values.iter().map(|v| {
                match v {
                    Value::Bool(val) => Some(*val),
                    Value::Null => None,
                    _ => None,
                }
            }).collect();
            Arc::new(BooleanArray::from(vals))
        }
        DataType::Null => {
            // For null arrays, we'll create an empty array of the appropriate type
            Arc::new(Int64Array::from(vec![None; values.len()]))
        }
        _ => return Err(DbError::Invalid("Unsupported data type for Arrow conversion".into())),
    };
    
    Ok(vec![array])
}

/// Convert a record batch to Parquet format and write to storage
pub fn write_record_batch_to_parquet<S: Storage + ?Sized>(
    storage: &S,
    space: &Space,
    key: Vec<u8>,
    batch: &RecordBatch,
) -> Result<()> {
    // Create an in-memory cursor for writing the Parquet file
    let cursor = InMemoryWriteableCursor::default();
    
    // Create a Parquet writer
    let schema = batch.schema();
    let props = WriterProperties::builder().build();
    let mut writer = ArrowWriter::try_new(cursor.clone(), schema, Some(props))
        .map_err(|e| DbError::Storage(format!("Failed to create Parquet writer: {}", e)))?;
    
    // Write the record batch
    writer.write(batch)
        .map_err(|e| DbError::Storage(format!("Failed to write record batch: {}", e)))?;
    
    // Close the writer to finalize the Parquet file
    writer.close()
        .map_err(|e| DbError::Storage(format!("Failed to close Parquet writer: {}", e)))?;
    
    // Get the Parquet data
    let parquet_data = cursor.data();
    
    // Store the Parquet data in the storage
    storage.put(space, key, parquet_data)
}

/// Read a Parquet file from storage and convert to a record batch
pub fn read_parquet_from_storage<S: Storage + ?Sized>(
    storage: &S,
    space: &Space,
    key: &[u8],
) -> Result<Option<RecordBatch>> {
    // Read the Parquet data from storage
    let parquet_data = match storage.get(space, key)? {
        Some(data) => data,
        None => return Ok(None),
    };
    
    // Create a cursor from the Parquet data
    let cursor = std::io::Cursor::new(parquet_data);
    
    // Create a Parquet reader
    let mut reader = parquet::arrow::ParquetRecordBatchReaderBuilder::try_new(cursor)
        .map_err(|e| DbError::Storage(format!("Failed to create Parquet reader: {}", e)))?
        .build()
        .map_err(|e| DbError::Storage(format!("Failed to build Parquet reader: {}", e)))?;
    
    // Read the first record batch
    let batch = reader.next()
        .ok_or_else(|| DbError::Storage("No record batches found in Parquet file".into()))?
        .map_err(|e| DbError::Storage(format!("Failed to read record batch: {}", e)))?;
    
    Ok(Some(batch))
}

#[cfg(test)]
mod tests {
    use super::*;
    use arrow::array::Int64Array;
    use std::sync::Arc;
    
    #[test]
    fn test_values_to_arrow_arrays() {
        let values = vec![
            Value::I64(1),
            Value::I64(2),
            Value::I64(3),
            Value::Null,
        ];
        
        let arrays = values_to_arrow_arrays(&values).unwrap();
        assert_eq!(arrays.len(), 1);
        
        let array = arrays[0].as_any().downcast_ref::<Int64Array>().unwrap();
        assert_eq!(array.len(), 4);
        assert_eq!(array.value(0), 1);
        assert_eq!(array.value(1), 2);
        assert_eq!(array.value(2), 3);
        assert!(array.is_null(3));
    }
}
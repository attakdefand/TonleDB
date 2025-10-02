//! Tests for Arrow functionality

use tonledb_arrow::{values_to_arrow_arrays, write_record_batch_to_parquet, read_parquet_from_storage};
use tonledb_core::{Value, Space, Storage};
use tonledb_storage::arc_inmem_with_wal;
use arrow::array::{Int64Array, RecordBatch};
use arrow::datatypes::{DataType, Field, Schema};
use std::sync::Arc;

#[test]
fn test_values_to_arrow_arrays_int64() {
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

#[test]
fn test_parquet_write_read() {
    let storage = arc_inmem_with_wal(None, 1000);
    let space = Space("test".to_string());
    let key = b"parquet_data".to_vec();
    
    // Create a simple record batch
    let schema = Arc::new(Schema::new(vec![
        Field::new("id", DataType::Int64, true),
        Field::new("value", DataType::Int64, true),
    ]));
    
    let id_array = Int64Array::from(vec![Some(1), Some(2), Some(3), None]);
    let value_array = Int64Array::from(vec![Some(10), Some(20), Some(30), None]);
    
    let batch = RecordBatch::try_new(
        schema,
        vec![Arc::new(id_array), Arc::new(value_array)],
    ).unwrap();
    
    // Write the record batch to Parquet and store it
    assert!(write_record_batch_to_parquet(&*storage, &space, key.clone(), &batch).is_ok());
    
    // Read the Parquet data back
    let read_batch = read_parquet_from_storage(&*storage, &space, &key).unwrap();
    assert!(read_batch.is_some());
    
    let read_batch = read_batch.unwrap();
    assert_eq!(read_batch.num_rows(), 4);
    assert_eq!(read_batch.num_columns(), 2);
}
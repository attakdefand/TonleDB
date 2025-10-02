//! Tests for secondary index functionality

use tonledb_core::{Db, Column, ColumnConstraint, DataType, TableSchema, TableConstraint};
use tonledb_storage::arc_inmem_with_wal;

#[test]
fn test_secondary_index_creation() {
    let storage = arc_inmem_with_wal(None, 1000);
    let db = Db::new(storage);
    
    // Create a simple table schema
    {
        let mut catalog = db.catalog.write();
        let table_schema = TableSchema {
            name: "users".to_string(),
            columns: vec![
                Column { 
                    name: "id".to_string(),
                    data_type: DataType::Integer,
                    constraints: vec![ColumnConstraint::PrimaryKey],
                },
                Column { 
                    name: "name".to_string(),
                    data_type: DataType::Text,
                    constraints: vec![ColumnConstraint::NotNull],
                },
                Column { 
                    name: "email".to_string(),
                    data_type: DataType::Text,
                    constraints: vec![ColumnConstraint::Unique],
                },
            ],
            pk: Some("id".to_string()),
            constraints: vec![],
        };
        catalog.tables.insert("users".to_string(), table_schema);
    }
    
    // Create an index
    assert!(db.create_index("users", "email", tonledb_core::IndexType::BTree, true).is_ok());
    
    // Verify the index was created
    let index = db.get_index("users", "email").unwrap();
    assert!(index.is_some());
    let index = index.unwrap();
    assert_eq!(index.table, "users");
    assert_eq!(index.column, "email");
    assert_eq!(index.index_type, tonledb_core::IndexType::BTree);
    assert!(index.is_unique);
}

#[test]
fn test_index_drop() {
    let storage = arc_inmem_with_wal(None, 1000);
    let db = Db::new(storage);
    
    // Create a simple table schema
    {
        let mut catalog = db.catalog.write();
        let table_schema = TableSchema {
            name: "users".to_string(),
            columns: vec![
                Column { 
                    name: "id".to_string(),
                    data_type: DataType::Integer,
                    constraints: vec![ColumnConstraint::PrimaryKey],
                },
                Column { 
                    name: "name".to_string(),
                    data_type: DataType::Text,
                    constraints: vec![ColumnConstraint::NotNull],
                },
                Column { 
                    name: "email".to_string(),
                    data_type: DataType::Text,
                    constraints: vec![ColumnConstraint::Unique],
                },
            ],
            pk: Some("id".to_string()),
            constraints: vec![],
        };
        catalog.tables.insert("users".to_string(), table_schema);
    }
    
    // Create an index
    assert!(db.create_index("users", "email", tonledb_core::IndexType::BTree, true).is_ok());
    
    // Drop the index
    assert!(db.drop_index("users", "email").is_ok());
    
    // Verify the index was dropped
    let index = db.get_index("users", "email").unwrap();
    assert!(index.is_none());
}
use sqlparser::{dialect::GenericDialect, parser::Parser};
use tonledb_core::{Db, DbError, Result, Space};
use tonledb_storage::index::SecondaryIndex;

const TBL_PREFIX: &str = "tbl/";

pub fn execute_sql(db: &Db, sql: &str) -> Result<serde_json::Value> {
    let stmts = Parser::parse_sql(&GenericDialect, sql).map_err(|e| DbError::Invalid(e.to_string()))?;
    if stmts.len() != 1 {
        return Err(DbError::Invalid("only single statement supported".into()));
    }
    
    match &stmts[0] {
        sqlparser::ast::Statement::Query(q) => {
            if let sqlparser::ast::SetExpr::Select(sel) = &*q.body {
                if sel.from.len() != 1 {
                    return Err(DbError::Invalid("SELECT from exactly one table".into()));
                }
                
                let tname = &sel.from[0].relation.to_string();
                let projection = &sel.projection;
                let selection = &sel.selection;
                let order_by = &q.order_by;
                let limit = &q.limit;
                
                let mut results = vec![];
                
                // Check if we can use an index for the query
                if let Some(index_scan) = try_index_scan(db, tname, selection)? {
                    // Use index scan
                    for row_key in index_scan.row_keys {
                        if let Some(row_data) = db.storage.get(&Space("data".into()), &row_key)? {
                            let mut obj: serde_json::Value = serde_json::from_slice(&row_data)
                                .map_err(|e| DbError::Storage(e.to_string()))?;
                            if let Some(sel) = selection {
                                if !eval_simple_where(&obj, &sel)? {
                                    continue;
                                }
                            }
                            results.push(obj);
                        }
                    }
                } else {
                    // Fallback: full scan with selection
                    let prefix = format!("{}{}{}", TBL_PREFIX, tname, "/").into_bytes();
                    let iter = db.storage.scan_prefix(&Space("data".into()), &prefix)?;
                    for (_, v) in iter { 
                        let mut obj: serde_json::Value = serde_json::from_slice(&v).map_err(|e| DbError::Storage(e.to_string()))?; 
                        if let Some(sel) = selection { 
                            if !eval_simple_where(&obj, &sel)? { 
                                continue; 
                            } 
                        } 
                        results.push(obj);
                    }
                }
                
                // Apply ORDER BY if specified
                if !order_by.is_empty() {
                    apply_order_by(&mut results, order_by)?;
                }
                
                // Apply LIMIT if specified
                if let Some(limit_expr) = limit {
                    if let Ok(limit_val) = value_of_placeholder(limit_expr) {
                        if let Ok(limit_num) = limit_val.parse::<usize>() {
                            results.truncate(limit_num);
                        }
                    }
                }
                
                // Apply projection to all results
                let mut out = vec![];
                for mut obj in results {
                    out.push(project_simple(&projection, &mut obj)?);
                }
                
                Ok(serde_json::Value::Array(out))
            } else {
                Err(DbError::Invalid("only SELECT supported".into()))
            }
        }
        _ => Err(DbError::Invalid("only SELECT supported".into())),
    }
}

fn eval_simple_where(row: &serde_json::Value, expr: &sqlparser::ast::Expr) -> Result<bool> {
    match expr { 
        sqlparser::ast::Expr::BinaryOp { left, op, right } => {
            let (l, r) = (value_of(row, left)?, value_of(row, right)?);
            
            match op {
                sqlparser::ast::BinaryOperator::Eq => Ok(l == r),
                sqlparser::ast::BinaryOperator::NotEq => Ok(l != r),
                sqlparser::ast::BinaryOperator::Gt => Ok(compare_values(&l, &r) == std::cmp::Ordering::Greater),
                sqlparser::ast::BinaryOperator::Lt => Ok(compare_values(&l, &r) == std::cmp::Ordering::Less),
                sqlparser::ast::BinaryOperator::GtEq => Ok(compare_values(&l, &r) != std::cmp::Ordering::Less),
                sqlparser::ast::BinaryOperator::LtEq => Ok(compare_values(&l, &r) != std::cmp::Ordering::Greater),
                _ => Err(DbError::Invalid(format!("Unsupported operator: {:?}", op))),
            }
        }
        sqlparser::ast::Expr::UnaryOp { op, expr } => {
            match op {
                sqlparser::ast::UnaryOperator::Not => {
                    let val = eval_simple_where(row, expr)?;
                    Ok(!val)
                }
                _ => Err(DbError::Invalid(format!("Unsupported unary operator: {:?}", op))),
            }
        }
        sqlparser::ast::Expr::Nested(expr) => {
            eval_simple_where(row, expr)
        }
        _ => Err(DbError::Invalid("Unsupported expression type".into())),
    }
}

fn value_of(row: &serde_json::Value, expr: &sqlparser::ast::Expr) -> Result<serde_json::Value> {
    match expr { 
        sqlparser::ast::Expr::Identifier(sqlparser::ast::Ident { value, .. }) => 
            Ok(row.get(value).cloned().unwrap_or(serde_json::Value::Null)), 
        sqlparser::ast::Expr::Value(v) => 
            Ok(lit_sql_to_json(v.clone())), 
        _ => Err(DbError::Invalid("unsupported expression".into())), 
    }
}

fn project_simple(proj: &Vec<sqlparser::ast::SelectItem>, row: &mut serde_json::Value) -> Result<serde_json::Value> {
    let obj = row.as_object().ok_or_else(|| DbError::Invalid("row not object".into()))?;
    if proj.len()==1 { 
        // Let's handle wildcard by checking if it's a wildcard pattern
        match &proj[0] {
            sqlparser::ast::SelectItem::Wildcard(_) => {
                return Ok(serde_json::Value::Object(obj.clone()));
            }
            _ => {}
        }
    }
    let mut out = serde_json::Map::new();
    for it in proj { 
        match it {
            sqlparser::ast::SelectItem::UnnamedExpr(sqlparser::ast::Expr::Identifier(sqlparser::ast::Ident { value, .. })) => { 
                if let Some(v) = obj.get(value) { 
                    out.insert(value.clone(), v.clone()); 
                } 
            }
            sqlparser::ast::SelectItem::ExprWithAlias { 
                expr: sqlparser::ast::Expr::Identifier(sqlparser::ast::Ident { value, .. }), 
                alias 
            } => { 
                if let Some(v) = obj.get(value) { 
                    out.insert(alias.value.clone(), v.clone()); 
                } 
            }
            _ => return Err(DbError::Invalid("projection supports identifiers only".into())), 
        } 
    }
    Ok(serde_json::Value::Object(out))
}

fn lit_sql_to_json(v: sqlparser::ast::Value) -> serde_json::Value { 
    match v {
        sqlparser::ast::Value::Number(n, _) => 
            serde_json::json!(n.parse::<f64>().unwrap()),
        sqlparser::ast::Value::SingleQuotedString(s) | sqlparser::ast::Value::DoubleQuotedString(s) => 
            serde_json::json!(s),
        sqlparser::ast::Value::Boolean(b) => 
            serde_json::json!(b),
        sqlparser::ast::Value::Null => 
            serde_json::Value::Null,
        _ => serde_json::Value::Null,
    } 
}

/// Represents an index scan operation
struct IndexScan {
    row_keys: Vec<Vec<u8>>,
    index_name: String,
}

/// Try to optimize the query using an index
fn try_index_scan(db: &Db, table_name: &str, selection: &Option<sqlparser::ast::Expr>) -> Result<Option<IndexScan>> {
    if let Some(expr) = selection {
        // Look for simple equality conditions that can use an index
        if let sqlparser::ast::Expr::BinaryOp { 
            left, 
            op: sqlparser::ast::BinaryOperator::Eq, 
            right 
        } = expr {
            // Check if the left side is a column identifier
            if let sqlparser::ast::Expr::Identifier(sqlparser::ast::Ident { value: column_name, .. }) = &**left {
                // Check if there's an index on this column
                let index_key = format!("{}.{}", table_name, column_name);
                if let Some(index_def) = db.catalog.read().indexes.get(&index_key) {
                    // Get the value to search for
                    if let Ok(search_value) = value_of_placeholder(right) {
                        let index = SecondaryIndex::new(
                            index_key.clone(),
                            index_def.table.clone(),
                            index_def.column.clone(),
                            index_def.is_unique,
                        );
                        
                        // Perform the index lookup
                        let row_keys = index.find_rows(&*db.storage, search_value.as_bytes())?;
                        
                        return Ok(Some(IndexScan {
                            row_keys,
                            index_name: index_key,
                        }));
                    }
                }
            }
        }
    }
    
    Ok(None)
}

/// Extract a value from an expression for index lookups
fn value_of_placeholder(expr: &sqlparser::ast::Expr) -> Result<String> {
    match expr {
        sqlparser::ast::Expr::Value(v) => {
            match v {
                sqlparser::ast::Value::Number(n, _) => Ok(n.clone()),
                sqlparser::ast::Value::SingleQuotedString(s) => Ok(s.clone()),
                sqlparser::ast::Value::DoubleQuotedString(s) => Ok(s.clone()),
                _ => Err(DbError::Invalid("unsupported value type for index lookup".into())),
            }
        }
        _ => Err(DbError::Invalid("unsupported expression for index lookup".into())),
    }
}

/// Compare two JSON values
fn compare_values(left: &serde_json::Value, right: &serde_json::Value) -> std::cmp::Ordering {
    match (left, right) {
        (serde_json::Value::Number(a), serde_json::Value::Number(b)) => {
            // Try to compare as f64 first
            if let (Some(a_f64), Some(b_f64)) = (a.as_f64(), b.as_f64()) {
                a_f64.partial_cmp(&b_f64).unwrap_or(std::cmp::Ordering::Equal)
            } else if let (Some(a_i64), Some(b_i64)) = (a.as_i64(), b.as_i64()) {
                a_i64.cmp(&b_i64)
            } else {
                std::cmp::Ordering::Equal
            }
        }
        (serde_json::Value::String(a), serde_json::Value::String(b)) => a.cmp(b),
        (serde_json::Value::Bool(a), serde_json::Value::Bool(b)) => a.cmp(b),
        _ => std::cmp::Ordering::Equal,
    }
}

/// Apply ORDER BY clause to results
fn apply_order_by(results: &mut Vec<serde_json::Value>, order_by: &[sqlparser::ast::OrderByExpr]) -> Result<()> {
    if order_by.is_empty() {
        return Ok(());
    }
    
    // For simplicity, we only support ordering by a single column
    if order_by.len() > 1 {
        return Err(DbError::Invalid("ORDER BY supports only single column".into()));
    }
    
    let order_expr = &order_by[0];
    if let sqlparser::ast::Expr::Identifier(sqlparser::ast::Ident { value, .. }) = &*order_expr.expr {
        let column_name = value.clone();
        let descending = !order_expr.asc.unwrap_or(true);
        
        results.sort_by(|a, b| {
            let a_val = a.get(&column_name).unwrap_or(&serde_json::Value::Null);
            let b_val = b.get(&column_name).unwrap_or(&serde_json::Value::Null);
            let cmp = compare_values(a_val, b_val);
            if descending { cmp.reverse() } else { cmp }
        });
    } else {
        return Err(DbError::Invalid("ORDER BY supports only column identifiers".into()));
    }
    
    Ok(())
}
